// Copyright (c) 2023 Espresso Systems (espressosys.com)
// This file is part of the sequencer-example-l2 repository.

// You should have received a copy of the MIT License
// along with the sequencer-example-l2 repository. If not, see <https://mit-license.org/>.

use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use async_std::sync::RwLock;
use clap::Parser;
use committable::Committable;
use contract_bindings::example_rollup::ExampleRollup;
use espresso_types::NamespaceId;
use ethers::{
    middleware::SignerMiddleware,
    providers::{Http, Middleware as _, Provider},
    signers::{coins_bip39::English, LocalWallet, MnemonicBuilder, Signer},
};
use example_l2::{
    api::{serve, APIOptions},
    executor::{run_executor, ExecutorOptions},
    seed::{SeedIdentity, INITIAL_BALANCE},
    state::State,
    Options, RollupVM,
};
use futures::join;
use rand::SeedableRng;
use rand_chacha::ChaChaRng;
use sequencer_utils::commitment_to_u256;
use std::{sync::Arc, time::Duration};
use strum::IntoEnumIterator;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    setup_logging();
    setup_backtrace();

    let opt = Options::parse();
    let vm = RollupVM::new(NamespaceId::from(1_u64));

    let mut initial_balances = vec![];
    for identity in SeedIdentity::iter() {
        let address = LocalWallet::new(&mut ChaChaRng::seed_from_u64(identity as u64)).address();
        initial_balances.push((address, INITIAL_BALANCE))
    }
    let state = Arc::new(RwLock::new(State::from_initial_balances(
        initial_balances,
        vm,
    )));

    let api_options = APIOptions {
        api_port: opt.api_port,
        espresso_url: opt.espresso_url.clone(),
    };

    let serve_api = async {
        serve(&api_options, state.clone()).await.unwrap();
    };

    let initial_state = { state.read().await.commit() };

    tracing::info!("Deploying Rollup contract");
    let provider = Provider::<Http>::try_from(opt.l1_http_provider.to_string())?
        .interval(Duration::from_secs(1));
    let chain_id = provider.get_chainid().await?.as_u64();
    let wallet = MnemonicBuilder::<English>::default()
        .phrase(opt.rollup_mnemonic.as_str())
        .index(opt.rollup_account_index)?
        .build()?
        .with_chain_id(chain_id);
    let l1_client = Arc::new(SignerMiddleware::new(provider.clone(), wallet));
    let rollup_contract = ExampleRollup::deploy(
        l1_client.clone(),
        (opt.light_client_address, commitment_to_u256(initial_state)),
    )?
    .send()
    .await?;

    tracing::info!("Launching Example Rollup API and Executor");
    let executor_options = ExecutorOptions {
        light_client_address: opt.light_client_address,
        l1_http_provider: opt.l1_http_provider.clone(),
        l1_ws_provider: opt.l1_ws_provider.clone(),
        rollup_address: rollup_contract.address(),
        rollup_account_index: opt.rollup_account_index,
        rollup_mnemonic: opt.rollup_mnemonic.clone(),
        espresso_url: opt.espresso_url.clone(),
        output_stream: None,
    };
    join!(run_executor(&executor_options, state.clone()), serve_api,);
    Ok(())
}
