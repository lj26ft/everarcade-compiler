use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettlementProof {
    pub proof_root: [u8; 32],
    pub verifier_set_hash: [u8; 32],
}
