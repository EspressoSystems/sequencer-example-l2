// Copyright (c) 2023 Espresso Systems (espressosys.com)
// This file is part of the sequencer-example-l2 repository.

// You should have received a copy of the MIT License
// along with the sequencer-example-l2 repository. If not, see <https://mit-license.org/>.

use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use async_std::sync::RwLock;
use clap::Parser;
use committable::Committable;
use espresso_types::NamespaceId;
use ethers::signers::{LocalWallet, Signer};
use example_l2::{
    api::{serve, APIOptions},
    executor::{run_executor, ExecutorOptions},
    seed::{SeedIdentity, INITIAL_BALANCE},
    state::State,
    utils::{create_provider, deploy_example_contract},
    Options, RollupVM,
};
use futures::join;
use rand::SeedableRng;
use rand_chacha::ChaChaRng;
use sequencer_utils::test_utils::TestL1System;
use std::sync::Arc;
use strum::IntoEnumIterator;

#[async_std::main]
async fn main() {
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
        sequencer_url: opt.sequencer_url.clone(),
    };

    let serve_api = async {
        serve(&api_options, state.clone()).await.unwrap();
    };

    let initial_state = { state.read().await.commit() };

    tracing::info!("Deploying Rollup contracts");
    let provider = create_provider(&opt.l1_http_provider);
    let test_system = TestL1System::new(provider, opt.hotshot_address)
        .await
        .unwrap();
    let rollup_contract = deploy_example_contract(&test_system, initial_state).await;

    let executor_options = ExecutorOptions {
        hotshot_address: opt.hotshot_address,
        l1_http_provider: opt.l1_http_provider.clone(),
        l1_ws_provider: opt.l1_ws_provider.clone(),
        rollup_address: rollup_contract.address(),
        rollup_account_index: opt.rollup_account_index,
        rollup_mnemonic: opt.rollup_mnemonic.clone(),
        sequencer_url: opt.sequencer_url.clone(),
        output_stream: None,
    };

    tracing::info!("Launching Example Rollup API and Executor");
    join!(run_executor(&executor_options, state.clone()), serve_api,);
}
