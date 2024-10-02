// Copyright (c) 2023 Espresso Systems (espressosys.com)
// This file is part of the sequencer-example-l2 repository.

// You should have received a copy of the MIT License
// along with the sequencer-example-l2 repository. If not, see <https://mit-license.org/>.

use crate::error::RollupError;
use crate::state::{Amount, Nonce};
use ethers::{abi::Address, signers::Signer, types::Signature};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub amount: Amount,
    pub destination: Address,
    pub nonce: Nonce,
}

impl Transaction {
    fn encode(&self) -> Vec<u8> {
        serde_json::to_string(&self)
            .expect("Serialization should not fail")
            .as_bytes()
            .to_vec()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SignedTransaction {
    pub transaction: Transaction,
    signature: Signature,
}

impl SignedTransaction {
    pub(crate) fn encode(&self) -> Vec<u8> {
        serde_json::to_string(&self)
            .expect("Serialization should not fail")
            .as_bytes()
            .to_vec()
    }

    pub(crate) fn decode(bytes: &[u8]) -> Option<Self> {
        serde_json::from_slice(bytes).ok()
    }

    pub fn recover(&self) -> Result<Address, RollupError> {
        let bytes = self.transaction.encode();
        self.signature
            .recover(bytes)
            .map_err(|_| RollupError::SignatureError)
    }

    pub async fn new(transaction: Transaction, wallet: &impl Signer) -> Self {
        let bytes = transaction.encode();
        let signature = wallet.sign_message(&bytes).await.unwrap();
        Self {
            signature,
            transaction,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::transaction::Transaction;
    use ethers::signers::{LocalWallet, Signer};

    use super::*;
    #[async_std::test]
    async fn test_transaction_signature() {
        let mut rng = rand::thread_rng();
        let alice = LocalWallet::new(&mut rng);
        let transaction = Transaction {
            amount: 100,
            destination: alice.address(),
            nonce: 1,
        };
        let signed_transaction = SignedTransaction::new(transaction, &alice).await;
        let recovered_address = signed_transaction
            .recover()
            .expect("Should recover address");
        assert_eq!(recovered_address, alice.address());
    }
}
