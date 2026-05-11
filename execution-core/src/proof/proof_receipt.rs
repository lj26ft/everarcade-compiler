use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofReceipt {
    pub receipt_hash: String,
    pub proof_hash: String,
    pub proof_system: String,
    pub aggregate: bool,
}
