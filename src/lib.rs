// Copyright (c) 2023 Espresso Systems (espressosys.com)
// This file is part of the sequencer-example-l2 repository.

// You should have received a copy of the MIT License
// along with the sequencer-example-l2 repository. If not, see <https://mit-license.org/>.

use clap::Parser;
use derive_more::{From, Into};
use espresso_types::NamespaceId;
use ethers::types::Address;
use surf_disco::Url;

pub mod api;
pub mod error;
pub mod executor;
mod prover;
pub mod seed;
pub mod state;
pub mod transaction;
pub mod utils;

#[derive(Parser, Clone, Debug)]
pub struct Options {
    /// Port where the Rollup API will be served
    #[clap(short, long, env = "ESPRESSO_DEMO_ROLLUP_PORT", default_value = "8084")]
    pub api_port: u16,

    /// URL of a HotShot sequencer node.
    #[clap(
        long,
        env = "ESPRESSO_SEQUENCER_URL",
        default_value = "http://0.0.0.0:24000/v0/"
    )]
    pub sequencer_url: Url,

    /// URL of layer 1 Ethereum JSON-RPC provider.
    #[clap(
        long,
        env = "ESPRESSO_DEMO_L1_HTTP_PROVIDER",
        default_value = "http://localhost:8545"
    )]
    pub l1_http_provider: Url,

    /// URL of layer 1 Ethereum JSON-RPC provider.
    #[clap(
        long,
        env = "ESPRESSO_DEMO_L1_WS_PROVIDER",
        default_value = "ws://localhost:8546"
    )]
    pub l1_ws_provider: Url,

    /// Address of HotShot contract on layer 1.
    #[clap(
        long,
        env = "ESPRESSO_DEMO_LIGHT_CLIENT_ADDRESS",
        default_value = "0x0c8e79f3534b00d9a3d4a856b665bf4ebc22f2ba"
    )]
    pub light_client_address: Address,

    /// Mnemonic phrase for the rollup wallet.
    ///
    /// This is the wallet that will be used to send batch proofs of transaction validity to the rollup
    /// contract. It must be funded with ETH on the layer 1.
    #[clap(
        long,
        env = "ESPRESSO_DEMO_ROLLUP_MNEMONIC",
        default_value = "test test test test test test test test test test test junk"
    )]
    pub rollup_mnemonic: String,

    /// Index of a funded account derived from mnemonic, desginating the account
    /// that will send proofs to the rollup contract
    #[clap(long, env = "ESPRESSO_DEMO_ROLLUP_ACCOUNT_INDEX", default_value = "1")]
    pub rollup_account_index: u32,
}

#[derive(Clone, Copy, Debug, Default, Into, From)]

pub struct RollupVM(NamespaceId);

impl RollupVM {
    pub fn new(namespace: NamespaceId) -> Self {
        RollupVM(namespace)
    }
}
