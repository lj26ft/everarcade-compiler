use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExecutionReceipt {
    pub contract_id: String,
    pub execution_id: String,
    pub previous_state_root: [u8; 32],
    pub new_state_root: [u8; 32],
    pub state_diff_hash: [u8; 32],
    pub fuel_used: u64,
    pub receipt_hash: [u8; 32],
    pub continuity_hash: [u8; 32],
}

impl ExecutionReceipt {
    pub fn immutable_hash(&self) -> [u8; 32] {
        let tuple = (
            &self.contract_id,
            &self.execution_id,
            self.previous_state_root,
            self.new_state_root,
            self.state_diff_hash,
            self.fuel_used,
            self.continuity_hash,
        );
        Sha256::digest(bincode::serialize(&tuple).expect("serialize receipt")).into()
    }
}
