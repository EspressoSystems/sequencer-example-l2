// Copyright (c) 2023 Espresso Systems (espressosys.com)
// This file is part of the sequencer-example-l2 repository.

// You should have received a copy of the MIT License
// along with the sequencer-example-l2 repository. If not, see <https://mit-license.org/>.

extern crate derive_more;

use committable::Commitment;
use contract_bindings::example_rollup as bindings;
use derive_more::Into;
use espresso_types::{Header, NsProof, SeqTypes};
use hotshot_query_service::availability::BlockHash;
use hotshot_query_service::VidCommon;
use sequencer_utils::commitment_to_u256;
use snafu::Snafu;

use crate::state::State;

/// An error that occurs while generating proofs.
#[derive(Clone, Debug, Snafu)]
pub enum ProofError {
    #[snafu(display("Proofs out of order at position {position} in batch proof. Previous proof ends in {new_state} but next proof starts in {old_state}."))]
    OutOfOrder {
        position: usize,
        new_state: Commitment<State>,
        old_state: Commitment<State>,
    },
}

/// A mock proof that state_commitment represents a valid state transition from
/// previous_state_commitment when the transactions in a given block are applied.
#[derive(Debug, Clone)]
pub(crate) struct Proof {
    block: BlockHash<SeqTypes>,
    old_state: Commitment<State>,
    new_state: Commitment<State>,
}

impl Proof {
    /// The namespace proof is a private input to the mock proof, showing that
    /// the proof of the state transition accounts for every transaction in the rollup's namespace
    ///
    /// Transaction data comes from the 'get_namespaced_leaves' method of the NamespaceProof interface.
    /// A real prover would incorporate this data during proof construction.
    pub fn generate(
        header: Header,
        state_commitment: Commitment<State>,
        previous_state_commitment: Commitment<State>,
        namespace_proof: Option<NsProof>,
        vid_common: VidCommon,
        block: BlockHash<SeqTypes>,
    ) -> Self {
        namespace_proof
            .unwrap()
            .verify(header.ns_table(), &header.payload_commitment(), &vid_common)
            .expect("Namespace proof failure, cannot continue");
        Self {
            block,
            old_state: previous_state_commitment,
            new_state: state_commitment,
        }
    }
}

/// A mock proof aggregating a batch of proofs for a range of blocks.
#[derive(Into)]
pub(crate) struct BatchProof {
    first_block: BlockHash<SeqTypes>,
    last_block: BlockHash<SeqTypes>,
    old_state: Commitment<State>,
    new_state: Commitment<State>,
}

impl BatchProof {
    /// Generate a proof of correct execution of a range of blocks.
    ///
    /// # Error
    ///
    /// `proofs` must contain, in order, a proof for each block in a consecutive chain. If it is
    /// out of order or not consecutive, an error will be returned.
    pub fn generate(proofs: &[Proof]) -> Result<BatchProof, ProofError> {
        for i in 0..proofs.len() - 1 {
            if proofs[i].new_state != proofs[i + 1].old_state {
                return Err(ProofError::OutOfOrder {
                    position: i,
                    new_state: proofs[i].new_state,
                    old_state: proofs[i].old_state,
                });
            }
        }

        Ok(BatchProof {
            first_block: proofs[0].block,
            last_block: proofs[proofs.len() - 1].clone().block,
            old_state: proofs[0].old_state,
            new_state: proofs[proofs.len() - 1].new_state,
        })
    }
}

impl From<BatchProof> for bindings::BatchProof {
    fn from(p: BatchProof) -> Self {
        Self {
            first_block: commitment_to_u256(p.first_block),
            last_block: commitment_to_u256(p.last_block),
            old_state: commitment_to_u256(p.old_state),
            new_state: commitment_to_u256(p.new_state),
        }
    }
}
