// Copyright (c) 2023 Espresso Systems (espressosys.com)
// This file is part of the sequencer-example-l2 repository.

// You should have received a copy of the MIT License
// along with the sequencer-example-l2 repository. If not, see <https://mit-license.org/>.

use crate::{state::State, transaction::SignedTransaction};
use async_std::sync::RwLock;
use committable::{Commitment, Committable};
use espresso_types::{NamespaceId, Transaction};
use ethers::abi::Address;
use futures::FutureExt;
use sequencer::SequencerApiVersion;
use std::io;
use std::sync::Arc;
use surf_disco::error::ClientError;
use surf_disco::{Client, Url};
use tide_disco::{error::ServerError, Api, App};

#[derive(Clone, Debug)]
pub struct APIOptions {
    pub api_port: u16,
    pub sequencer_url: Url,
}

async fn submit_transaction(
    submit_url: Url,
    transaction: SignedTransaction,
) -> Result<Commitment<Transaction>, ServerError> {
    let raw_tx = transaction.encode();
    let txn = Transaction::new(NamespaceId::from(1_u64), raw_tx);
    let client: Client<ClientError, SequencerApiVersion> = Client::new(submit_url.clone());
    client.connect(None).await;
    client
        .post::<()>("submit/submit")
        .body_json(&txn)
        .unwrap()
        .send()
        .await
        .unwrap();
    let tx_hash = txn.commit();
    Ok(tx_hash)
}

pub async fn serve(options: &APIOptions, state: Arc<RwLock<State>>) -> io::Result<()> {
    type StateType = Arc<RwLock<State>>;
    let error_mapper = |err| io::Error::new(io::ErrorKind::Other, err);
    let APIOptions {
        api_port,
        sequencer_url,
        ..
    } = options.clone();
    let mut app = App::<StateType, ServerError>::with_state(state);
    let toml = toml::from_str::<toml::Value>(include_str!("api.toml"))
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
    let mut api =
        Api::<StateType, ServerError, SequencerApiVersion>::new(toml).map_err(error_mapper)?;

    api.post("submit",  move|req, _state| {
        let url = sequencer_url.clone();
        async move {
            let transaction = req
                .body_auto::<SignedTransaction, SequencerApiVersion>(SequencerApiVersion {}).
            map_err(|_| ServerError {
                status: tide_disco::StatusCode::BAD_REQUEST,
                message: "Malformed transaction. Ensure that the transaction is a JSON serialized SignedTransaction".into()
            })?;
             submit_transaction(url, transaction).await
        }
        .boxed()
    })
    .map_err(error_mapper)?;

    api.get("balance", |req, state| {
        async move {
            let address_str = req.string_param("address")?;
            let address = address_str.parse::<Address>().
            map_err(|_| ServerError {
                status: tide_disco::StatusCode::BAD_REQUEST,
                message: "Malformed address. Ensure that the address is valid hex encoded Ethereum address.".into()
            })?;
            let balance = state.get_balance(&address);
            Ok(balance)
        }
        .boxed()
    })
    .map_err(error_mapper)?;

    api.get("nonce", |req, state| {
        async move {
            let address_str = req.string_param("address")?;
            let address = address_str.parse::<Address>().
            map_err(|_| ServerError {
                status: tide_disco::StatusCode::BAD_REQUEST,
                message: "Malformed address. Ensure that the address is valid hex encoded Ethereum address.".into()
            })?;
            let nonce = state.get_nonce(&address);
            Ok(nonce)
        }
        .boxed()
    })
    .map_err(error_mapper)?;

    app.register_module("rollup", api)
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
    app.serve(format!("0.0.0.0:{}", api_port), SequencerApiVersion {})
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transaction::Transaction;
    use crate::RollupVM;
    use async_std::task::spawn;
    use espresso_types::{MockSequencerVersions, NamespaceId, Transaction as SeqTransaction};
    use ethers::signers::{LocalWallet, Signer};
    use ethers::utils::Anvil;
    use portpicker::pick_unused_port;
    use rand::SeedableRng;
    use rand_chacha::ChaChaRng;
    use sequencer::api::test_helpers::{TestNetwork, TestNetworkConfigBuilder};
    use sequencer::api::Options;
    use sequencer::testing::wait_for_decide_on_handle;
    use sequencer::testing::TestConfigBuilder;
    use surf_disco::Client;

    const GENESIS_BALANCE: u64 = 9999;

    #[async_std::test]
    async fn query_test() {
        let mut rng = rand::thread_rng();
        let genesis_wallet = LocalWallet::new(&mut rng);
        let vm = RollupVM::new(NamespaceId::from(1_u32));
        let genesis_address = genesis_wallet.address();
        let state = Arc::new(RwLock::new(State::from_initial_balances(
            [(genesis_address, GENESIS_BALANCE)],
            vm,
        )));
        let port = pick_unused_port().expect("No ports free");
        let api_url: Url = format!("http://localhost:{port}").parse().unwrap();
        let client: Client<ClientError, SequencerApiVersion> = Client::new(api_url.clone());
        let options = APIOptions {
            api_port: port,
            sequencer_url: api_url,
        };

        spawn(async move { serve(&options, state).await });

        client.connect(None).await;

        // Fetch genesis block balance
        let balance = client
            .get::<u64>(&format!("rollup/balance/{:?}", genesis_address))
            .send()
            .await
            .unwrap();

        assert_eq!(balance, GENESIS_BALANCE);
    }

    #[async_std::test]
    async fn submit_test() {
        // Start a sequencer network.
        let port = portpicker::pick_unused_port().unwrap();

        let options = Options::with_port(port).submit(Default::default());
        let anvil = Anvil::new().spawn();
        let l1 = anvil.endpoint().parse().unwrap();
        let network_config = TestConfigBuilder::default().l1_url(l1).build();
        let config = TestNetworkConfigBuilder::default()
            .api_config(options)
            .network_config(network_config)
            .build();
        let network = TestNetwork::new(config, MockSequencerVersions::new()).await;
        let mut events = network.server.event_stream().await;

        // Start the Rollup API
        let vm = RollupVM::new(NamespaceId::from(1_u64));

        let api_port = pick_unused_port().unwrap();
        let genesis_wallet = LocalWallet::new(&mut ChaChaRng::seed_from_u64(0));
        let genesis_address = genesis_wallet.address();
        let state = Arc::new(RwLock::new(State::from_initial_balances(
            [(genesis_address, GENESIS_BALANCE)],
            vm,
        )));

        let options = APIOptions {
            api_port,
            sequencer_url: format!("http://localhost:{port}").parse().unwrap(),
        };

        spawn(async move { serve(&options, state).await });

        // Create a transaction
        let transaction = Transaction {
            amount: 100,
            destination: genesis_address,
            nonce: 1,
        };
        let signed_transaction = SignedTransaction::new(transaction, &genesis_wallet).await;

        // Submit the transaction
        let api_url = format!("http://localhost:{api_port}").parse().unwrap();
        let api_client: Client<ClientError, SequencerApiVersion> = Client::new(api_url);
        api_client.connect(None).await;
        api_client
            .post::<()>("rollup/submit")
            .body_json(&signed_transaction)
            .unwrap()
            .send()
            .await
            .unwrap();

        // Wait for a Decide event containing transaction matching the one we sent
        let raw_tx = signed_transaction.encode();
        let txn = SeqTransaction::new(vm.0, raw_tx);
        wait_for_decide_on_handle(&mut events, &txn).await;
    }
}
