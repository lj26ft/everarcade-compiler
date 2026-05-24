use serde::{Deserialize, Serialize};

use crate::hashing::sha256;

use super::serialization::canonical_bytes;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionCheckpoint {
    pub prior_checkpoint_hash: String,
    pub previous_state_root: String,
    pub new_state_root: String,
    pub execution_receipt_hash: String,
    pub mutation_hash: String,
    pub module_hash: String,
    pub checkpoint_hash: String,
}

impl ExecutionCheckpoint {
    pub fn new(
        prior_checkpoint_hash: String,
        previous_state_root: String,
        new_state_root: String,
        execution_receipt_hash: String,
        mutation_hash: String,
        module_hash: String,
    ) -> anyhow::Result<Self> {
        #[derive(Serialize)]
        struct HashInput<'a> {
            prior_checkpoint_hash: &'a str,
            previous_state_root: &'a str,
            new_state_root: &'a str,
            execution_receipt_hash: &'a str,
            mutation_hash: &'a str,
            module_hash: &'a str,
        }

        let input = HashInput {
            prior_checkpoint_hash: &prior_checkpoint_hash,
            previous_state_root: &previous_state_root,
            new_state_root: &new_state_root,
            execution_receipt_hash: &execution_receipt_hash,
            mutation_hash: &mutation_hash,
            module_hash: &module_hash,
        };
        let checkpoint_hash = hex::encode(sha256(&canonical_bytes(&input)?));
        Ok(Self {
            prior_checkpoint_hash,
            previous_state_root,
            new_state_root,
            execution_receipt_hash,
            mutation_hash,
            module_hash,
            checkpoint_hash,
        })
    }
}
