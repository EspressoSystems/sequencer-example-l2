// Copyright (c) 2023 Espresso Systems (espressosys.com)
// This file is part of the sequencer-example-l2 repository.

// You should have received a copy of the MIT License
// along with the sequencer-example-l2 repository. If not, see <https://mit-license.org/>.

use std::time::Duration;

use crate::state::State;
use commit::Commitment;
use contract_bindings::example_rollup::ExampleRollup;
use ethers::{prelude::*, providers::Provider};
use sequencer_utils::{commitment_to_u256, test_utils::TestL1System, Signer};
use surf_disco::Url;

pub type ExampleRollupContract = ExampleRollup<Signer>;

pub async fn deploy_example_contract(
    test_l1: &TestL1System,
    initial_state: Commitment<State>,
) -> ExampleRollupContract {
    ExampleRollup::deploy(
        test_l1.clients.deployer.provider.clone(),
        (test_l1.hotshot.address(), commitment_to_u256(initial_state)),
    )
    .unwrap()
    .send()
    .await
    .unwrap()
}

pub fn create_provider(l1_url: &Url) -> Provider<Http> {
    let mut provider = Provider::try_from(l1_url.to_string()).unwrap();
    provider.set_interval(Duration::from_millis(10));
    provider
}
