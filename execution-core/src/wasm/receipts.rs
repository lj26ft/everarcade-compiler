use serde::{Deserialize, Serialize};

use crate::hashing::sha256;

use super::execution::ExecutionStatus;
use super::serialization::{canonical_bytes, canonical_hash};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeterministicExecutionReceipt {
    pub module_hash: String,
    pub engine_config_hash: String,
    pub abi_request_hash: String,
    pub previous_state_root: String,
    pub new_state_root: String,
    pub mutation_hash: String,
    pub fuel_budget: u64,
    pub fuel_used: u64,
    pub execution_status: ExecutionStatus,
    pub stdout_hash: String,
    pub continuity_proof_hash: String,
}

impl DeterministicExecutionReceipt {
    pub fn canonical_bytes(&self) -> anyhow::Result<Vec<u8>> {
        canonical_bytes(self)
    }

    pub fn receipt_hash(&self) -> anyhow::Result<String> {
        Ok(hex::encode(canonical_hash(self)?))
    }

    pub fn derive_continuity_proof_hash(&self) -> anyhow::Result<String> {
        #[derive(Serialize)]
        struct Continuity<'a> {
            module_hash: &'a str,
            engine_config_hash: &'a str,
            abi_request_hash: &'a str,
            previous_state_root: &'a str,
            new_state_root: &'a str,
            mutation_hash: &'a str,
            execution_status: &'a ExecutionStatus,
            stdout_hash: &'a str,
        }

        let continuity = Continuity {
            module_hash: &self.module_hash,
            engine_config_hash: &self.engine_config_hash,
            abi_request_hash: &self.abi_request_hash,
            previous_state_root: &self.previous_state_root,
            new_state_root: &self.new_state_root,
            mutation_hash: &self.mutation_hash,
            execution_status: &self.execution_status,
            stdout_hash: &self.stdout_hash,
        };
        Ok(hex::encode(sha256(&canonical_bytes(&continuity)?)))
    }
}
