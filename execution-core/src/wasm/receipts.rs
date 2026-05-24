use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionReceipt {
    pub module_hash: String,
    pub input_hash: String,
    pub output_hash: String,
    pub mutation_root: String,
    pub state_root: String,
    pub fuel_consumed: u64,
    pub continuity_root: String,
    pub execution_status: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionProof {
    pub receipt_hash: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionResultEnvelope {
    pub receipt: ExecutionReceipt,
    pub proof: ExecutionProof,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionContinuityRecord {
    pub continuity_root: String,
    pub checkpoint_height: u64,
}
