use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExecutionReceipt {
    pub previous_state_root: String,
    pub new_state_root: String,
    pub execution_root: String,
    pub receipt_hash: String,
    pub abi_version: String,
}
