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
use hotshot_contract_bindings::light_client::{LightClient, NewStateFilter};
use hotshot_query_service::availability::{PayloadQueryData, VidCommonQueryData};
use sequencer::api::endpoints::NamespaceProofQueryData;
use sequencer::SequencerApiVersion;
use sequencer_utils::{commitment_to_u256, contract_send};
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
    pub light_client_address: Address,
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
        light_client_address,
        rollup_address,
        rollup_mnemonic,
        output_stream,
    } = opt;

    let query_service_url = sequencer_url.join("availability").unwrap();
    let hotshot = HotShotClient::new(query_service_url.clone());

    hotshot.connect(None).await;

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
    let light_client = LightClient::new(*light_client_address, Arc::new(socket_provider));

    let filter = light_client.new_state_filter().from_block(0);

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

    while let Some(event) = commits_stream.next().await {
        tracing::info!(" new state event received {:?}", event);
        let (_view_num, block_height, _block_comm_root) = match event {
            Ok(NewStateFilter {
                view_num: _view_num,
                block_height,
                block_comm_root: _block_comm_root,
            }) => (_view_num, block_height, _block_comm_root),
            Err(err) => {
                tracing::error!("Error in Light client  stream, retrying: {err}");
                continue;
            }
        };

        // Full block content may not be available immediately so wait for all blocks to be ready
        // before building the batch proof
        let headers: Vec<Header> = header_stream
            .by_ref()
            .take(block_height as usize)
            .map(|result| result.expect("Error fetching block header"))
            .collect()
            .await;

        // Execute new blocks, generating proofs.
        let mut proofs = vec![];

        for header in headers.clone().into_iter() {
            let namespace_proof_query: Result<NamespaceProofQueryData, ClientError> = hotshot
                .get::<NamespaceProofQueryData>(&format!(
                    "block/{}/namespace/{}",
                    header.height(),
                    namespace_id
                ))
                .send()
                .await;

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
                stream.send_async((block_height, state.clone())).await.ok();
            }
        }

        // Compute an aggregate proof.
        if proofs.is_empty() {
            continue;
        }
        let proof = BatchProof::generate(&proofs).expect("Error generating batch proof");
        let state_comm = commitment_to_u256(state.read().await.commit());

        let proof = example_rollup::BatchProof::from(proof);
        let call =
            rollup_contract.verify_blocks(headers.len().try_into().unwrap(), state_comm, proof);
        let res = contract_send::<_, _, ExampleRollupErrors>(&call).await;
        if let Err(err) = res {
            tracing::warn!("Failed to submit proof to contract, retrying: {err}");
            sleep(Duration::from_secs(1)).await;
        } else {
            tracing::info!("Proof submitted successfully");
        }
    }
}
