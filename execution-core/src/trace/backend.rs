use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::trace::trace::ExecutionTrace;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExecutionProof {
    pub backend_id: String,
    pub trace_root: String,
    pub proof_bytes: Vec<u8>,
}

pub trait ProofBackend: Send + Sync {
    fn backend_id(&self) -> &'static str;
    fn generate_proof(&self, trace: &ExecutionTrace) -> Result<ExecutionProof>;
    fn verify_proof(&self, proof: &ExecutionProof) -> Result<bool>;
}
