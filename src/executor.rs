// Copyright (c) 2023 Espresso Systems (espressosys.com)
// This file is part of the sequencer-example-l2 repository.

// You should have received a copy of the MIT License
// along with the sequencer-example-l2 repository. If not, see <https://mit-license.org/>.

use crate::prover::BatchProof;
use crate::state::State;
use async_compatibility_layer::async_primitives::broadcast::BroadcastSender;
use async_std::sync::{Arc, RwLock};
use async_std::task::sleep;
use committable::Committable;
use contract_bindings::example_rollup::{self, ExampleRollup, ExampleRollupErrors};
use espresso_types::{Header, NamespaceId, SeqTypes};
use ethers::core::k256::ecdsa::SigningKey;
use ethers::prelude::*;
use ethers::{
    prelude::SignerMiddleware,
    providers::{Http, Middleware, Provider},
    signers::{coins_bip39::English, MnemonicBuilder},
    types::Address,
};
use hotshot_contract_bindings::hot_shot::{HotShot, NewBlocksFilter};
use hotshot_query_service::availability::{PayloadQueryData, VidCommonQueryData};
use sequencer::api::endpoints::NamespaceProofQueryData;
use sequencer::SequencerApiVersion;
use sequencer_utils::{commitment_to_u256, contract_send, u256_to_commitment};
use std::time::Duration;
use surf_disco::error::ClientError;
use surf_disco::Url;

pub async fn connect_rpc(
    provider: &Url,
    mnemonic: &str,
    index: u32,
    chain_id: Option<u64>,
    polling_interval: Option<Duration>,
) -> Option<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>> {
    let mut provider = match Provider::try_from(provider.to_string()) {
        Ok(provider) => provider,
        Err(err) => {
            tracing::error!("error connecting to RPC {}: {}", provider, err);
            return None;
        }
    };
    tracing::info!("Connected to RPC {}", provider.url());

    if let Some(interval) = polling_interval {
        provider.set_interval(interval);
    }
    tracing::info!("RPC Polling interval is {:?}", provider.get_interval());

    let chain_id = match chain_id {
        Some(id) => id,
        None => match provider.get_chainid().await {
            Ok(id) => id.as_u64(),
            Err(err) => {
                tracing::error!("error getting chain ID: {}", err);
                return None;
            }
        },
    };
    tracing::info!("Chain ID is {}", chain_id);

    let mnemonic = match MnemonicBuilder::<English>::default()
        .phrase(mnemonic)
        .index(index)
    {
        Ok(mnemonic) => mnemonic,
        Err(err) => {
            tracing::error!("error building walletE: {}", err);
            return None;
        }
    };
    let wallet = match mnemonic.build() {
        Ok(wallet) => wallet,
        Err(err) => {
            tracing::error!("error opening wallet: {}", err);
            return None;
        }
    };
    let wallet = wallet.with_chain_id(chain_id);
    Some(SignerMiddleware::new(provider, wallet))
}

type HotShotClient = surf_disco::Client<ClientError, SequencerApiVersion>;

#[derive(Clone, Debug)]
pub struct ExecutorOptions {
    pub sequencer_url: Url,
    pub l1_http_provider: Url,
    pub l1_ws_provider: Url,
    pub rollup_account_index: u32,
    pub rollup_mnemonic: String,
    pub hotshot_address: Address,
    pub rollup_address: Address,
    pub output_stream: Option<BroadcastSender<(u64, State)>>,
}

/// Runs the executor service, which is responsible for:
/// 1) Fetching blocks of ordered transactions from HotShot and applying them to the Rollup State.
/// 2) Submitting mock proofs to the Rollup Contract.
pub async fn run_executor(opt: &ExecutorOptions, state: Arc<RwLock<State>>) {
    let ExecutorOptions {
        rollup_account_index,
        sequencer_url,
        l1_http_provider,
        l1_ws_provider,
        hotshot_address,
        rollup_address,
        rollup_mnemonic,
        output_stream,
    } = opt;

    let query_service_url = sequencer_url.join("v0/availability").unwrap();
    let hotshot = HotShotClient::builder(query_service_url.clone())
        .set_timeout(Some(Duration::from_secs(10)))
        .build();

    hotshot.connect(None).await;
    println!("query_service_url: {}", query_service_url.as_str());
    println!("connected to hotshot");

    // Connect to the layer one HotShot contract.
    let l1 = connect_rpc(
        l1_http_provider,
        rollup_mnemonic,
        *rollup_account_index,
        None,
        None,
    )
    .await
    .expect("unable to connect to L1, hotshot commitment task exiting");

    // Create a socket connection to the L1 to subscribe to contract events
    // This assumes that the L1 node supports both HTTP and Websocket connections
    let socket_provider = Provider::<Ws>::connect(l1_ws_provider)
        .await
        .expect("Unable to make websocket connection to L1");

    let rollup_contract = ExampleRollup::new(*rollup_address, Arc::new(l1));
    println!("hotshot address inside run_executor: {:?}", hotshot_address);
    let hotshot_contract = HotShot::new(*hotshot_address, Arc::new(socket_provider));

    let filter = hotshot_contract
        .new_blocks_filter()
        .from_block(0)
        // Ethers does not set the contract address on filters created via contract bindings. This
        // seems like a bug and I have reported it: https://github.com/gakonst/ethers-rs/issues/2528.
        // In the mean time we can work around by setting the address manually.
        .address(hotshot_contract.address().into());

    let mut commits_stream = filter
        .subscribe()
        .await
        .expect("Unable to subscribe to L1 log stream");

    let mut header_stream = hotshot
        .socket("stream/headers/0")
        .subscribe::<Header>()
        .await
        .expect("Unable to subscribe to HotShot block header stream");
    let namespace_id: NamespaceId = state.read().await.vm.into();
    println!("created header stream: {namespace_id:?}");

    while let Some(event) = commits_stream.next().await {
        println!("inside the commits loop: {event:?}");
        let (first_block, num_blocks) = match event {
            Ok(NewBlocksFilter {
                first_block_number,
                num_blocks,
            }) => (first_block_number, num_blocks.as_u64()),
            Err(err) => {
                tracing::error!("Error in HotShot block stream, retrying: {err}");
                continue;
            }
        };

        // Full block content may not be available immediately so wait for all blocks to be ready
        // before building the batch proof
        let headers: Vec<Header> = header_stream
            .by_ref()
            .take(num_blocks as usize)
            .map(|result| result.expect("Error fetching block header"))
            .collect()
            .await;

        // Execute new blocks, generating proofs.
        let mut proofs = vec![];
        tracing::info!(
            "executing blocks {}-{}, state is {}",
            first_block,
            first_block + num_blocks - 1,
            state.read().await.commit()
        );
        for (i, header) in headers.into_iter().enumerate() {
            let commitment = hotshot_contract
                .commitments(first_block + i)
                .call()
                .await
                .expect("Unable to read commitment");
            let block_commitment =
                u256_to_commitment(commitment).expect("Unable to deserialize block commitment");

            if header.commit() != block_commitment {
                panic!("Block commitment does not match hash of received block, the executor cannot continue");
            }

            let namespace_proof_query: Result<NamespaceProofQueryData, ClientError> = hotshot
                .get::<NamespaceProofQueryData>(&format!(
                    "block/{}/namespace/{}",
                    header.height(),
                    namespace_id
                ))
                .send()
                .await;

            // Weird bug where the only time this errors is when the returned response
            // isn't of the correct type. It actually returns some random metadata about the
            // request itself, instead of a response type.
            if namespace_proof_query.is_err() {
                continue;
            }

            let namespace_proof = namespace_proof_query.unwrap().proof;
            if namespace_proof.is_none() {
                continue;
            }

            let vid_common = hotshot
                .get::<VidCommonQueryData<SeqTypes>>(&format!("vid/common/{}", header.height()))
                .send()
                .await
                .unwrap();

            let block_hash: PayloadQueryData<SeqTypes> = hotshot
                .get(&format!("payload/{}", header.height()))
                .send()
                .await
                .unwrap();

            let mut state = state.write().await;
            proofs.push(
                state
                    .execute_block(
                        header,
                        namespace_proof,
                        vid_common.common().clone(),
                        block_hash.block_hash(),
                    )
                    .await,
            );
            if let Some(stream) = &output_stream {
                stream
                    .send_async((first_block.as_u64() + (i as u64), state.clone()))
                    .await
                    .ok();
            }
        }

        // Compute an aggregate proof.
        if proofs.is_empty() {
            continue;
        }
        let proof = BatchProof::generate(&proofs).expect("Error generating batch proof");
        let state_comm = commitment_to_u256(state.read().await.commit());

        let proof = example_rollup::BatchProof::from(proof);
        let call = rollup_contract.verify_blocks(num_blocks, state_comm, proof);
        while let Err(err) = contract_send::<_, _, ExampleRollupErrors>(&call).await {
            tracing::warn!("Failed to submit proof to contract, retrying: {err}");
            sleep(std::time::Duration::from_secs(1)).await;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::state::{Amount, Nonce};
    use crate::transaction::{SignedTransaction, Transaction};
    use crate::utils::{create_provider, deploy_example_contract, ExampleRollupContract};
    use crate::RollupVM;

    use super::*;
    use async_compatibility_layer::{
        async_primitives::broadcast,
        logging::{setup_backtrace, setup_logging},
    };
    use async_std::task::spawn;
    use derivative::Derivative;
    use espresso_types::{MockSequencerVersions, NamespaceId, Transaction as SeqTransaction};
    use ethers::prelude::k256::ecdsa::SigningKey;
    use ethers::providers::{Middleware, Provider};
    use ethers::signers::{LocalWallet, Signer};
    use portpicker::pick_unused_port;
    use rand::SeedableRng;
    use rand_chacha::ChaChaRng;
    use sequencer::api::test_helpers::{TestNetwork, TestNetworkConfigBuilder};
    use sequencer::testing::TestConfigBuilder;
    use sequencer::{
        api::options::Options,
        hotshot_commitment::{run_hotshot_commitment_task, CommitmentTaskOptions},
        persistence::fs,
        SequencerApiVersion,
    };
    use sequencer_utils::{test_utils::TestL1System, Anvil, AnvilOptions};
    use std::time::Duration;
    use surf_disco::error::ClientError;
    use surf_disco::{Client, Url};
    use tempfile::TempDir;

    #[derive(Clone, Derivative)]
    #[derivative(Debug)]
    struct TestRollupInstance {
        contract: ExampleRollupContract,
        vm: RollupVM,
        socket_provider: Provider<Ws>,
        l1_url: Url,
        alice: Wallet<SigningKey>,
        state: Arc<RwLock<State>>,
        bob: Wallet<SigningKey>,
        #[derivative(Debug = "ignore")]
        executor_send: BroadcastSender<(u64, State)>,
    }

    impl TestRollupInstance {
        pub async fn launch(
            l1_url: Url,
            namespace_id: NamespaceId,
            alice: Wallet<SigningKey>,
            bob: Wallet<SigningKey>,
            test_l1: &TestL1System,
        ) -> Self {
            // Create mock rollup state
            let vm = RollupVM::new(namespace_id);
            let state = State::from_initial_balances([(alice.address(), 9999)], vm);
            let initial_state = state.commit();
            let state = Arc::new(RwLock::new(state));
            let mut ws_url = l1_url.clone();
            ws_url.set_scheme("ws").unwrap();
            let socket_provider = Provider::<Ws>::connect(ws_url).await.unwrap();
            let rollup_contract = deploy_example_contract(test_l1, initial_state).await;
            let (executor_send, _) = broadcast::channel();

            Self {
                contract: rollup_contract,
                vm,
                socket_provider,
                alice,
                l1_url,
                bob,
                state,
                executor_send,
            }
        }

        pub async fn test_transaction(&self, amount: Amount, nonce: Nonce) -> SeqTransaction {
            let txn = Transaction {
                amount,
                destination: self.bob.address(),
                nonce,
            };
            let txn = SignedTransaction::new(txn, &self.alice).await;
            SeqTransaction::new(self.vm.0, txn.encode())
        }
    }

    async fn spawn_anvil() -> Anvil {
        let anvil = AnvilOptions::default()
            .block_time(Duration::from_secs(1))
            .spawn()
            .await;

        // When we are running a local Anvil node, as in tests, some endpoints (e.g. eth_feeHistory)
        // do not work until at least one block has been mined. Wait until the fee history endpoint
        // works.
        let provider = create_provider(&anvil.url());
        while let Err(err) = provider.fee_history(1, BlockNumber::Latest, &[]).await {
            tracing::warn!("RPC is not ready: {err}");
            sleep(Duration::from_secs(1)).await;
        }

        anvil
    }

    const TEST_MNEMONIC: &str = "test test test test test test test test test test test junk";
    #[async_std::test]
    async fn test_execute() {
        setup_logging();
        setup_backtrace();

        let anvil = spawn_anvil().await;
        let alice = LocalWallet::new(&mut ChaChaRng::seed_from_u64(0));
        let bob = LocalWallet::new(&mut ChaChaRng::seed_from_u64(1));

        // Deploy hotshot contract
        let provider = create_provider(&anvil.url());
        let test_l1 = TestL1System::deploy(provider).await.unwrap();

        // Start a test Rollup instance
        let test_rollup = TestRollupInstance::launch(
            anvil.url().clone(),
            NamespaceId::from(10_u64),
            alice,
            bob.clone(),
            &test_l1,
        )
        .await;

        // Start a test HotShot configuration
        let sequencer_port = pick_unused_port().unwrap();
        let network_config = TestConfigBuilder::default().l1_url(anvil.url()).build();
        let tmp_dir = TempDir::new().unwrap();
        let storage_path = tmp_dir.path().join("tmp_storage");
        let options = Options::with_port(sequencer_port)
            .submit(Default::default())
            .query_fs(Default::default(), fs::Options::new(storage_path))
            .status(Default::default());

        let config = TestNetworkConfigBuilder::default()
            .api_config(options)
            .network_config(network_config)
            .build();

        let network = TestNetwork::new(config, MockSequencerVersions::new()).await;
        let txn = test_rollup.test_transaction(100, 1).await;
        let sequencer_url: Url = format!("http://localhost:{sequencer_port}")
            .parse()
            .unwrap();
        let client: Client<ClientError, SequencerApiVersion> = Client::new(sequencer_url.clone());

        client.connect(None).await;
        client
            .post::<()>("submit/submit")
            .body_json(&txn)
            .unwrap()
            .send()
            .await
            .unwrap();

        // Spawn hotshot commitment and executor tasks
        let hotshot_opt = CommitmentTaskOptions {
            l1_provider: anvil.url(),
            sequencer_mnemonic: TEST_MNEMONIC.to_string(),
            sequencer_account_index: test_l1.clients.funded[0].index,
            hotshot_address: test_l1.hotshot.address(),
            l1_chain_id: None,
            query_service_url: Some(sequencer_url.clone().join("v0").unwrap()),
            request_timeout: Duration::from_secs(10),
            delay: Some(Duration::from_secs(1)),
        };

        let rollup_opt = ExecutorOptions {
            sequencer_url,
            rollup_account_index: test_l1.clients.funded[1].index,
            l1_http_provider: anvil.url(),
            l1_ws_provider: anvil.ws_url(),
            rollup_mnemonic: TEST_MNEMONIC.to_string(),
            hotshot_address: test_l1.hotshot.address(),
            rollup_address: test_rollup.contract.address(),
            output_stream: Some(test_rollup.executor_send.clone()),
        };

        let state_lock = test_rollup.state.clone();
        spawn(
            async move { run_hotshot_commitment_task::<SequencerApiVersion>(&hotshot_opt).await },
        );
        spawn(async move { run_executor(&rollup_opt, state_lock).await });

        let mut events = network.server.event_stream().await;

        loop {
            // Wait for an event. This stream should not end until our events have been processed.
            events.next().await.unwrap();
            let bob_balance = test_rollup
                .state
                .read()
                .await
                .get_balance(&bob.address().clone());
            if bob_balance == 100 {
                tracing::info!("Bob's balance was updated");
                break;
            } else {
                tracing::info!("Bob's balance is {bob_balance}/100");
            }
        }
    }
}
