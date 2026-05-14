use anyhow::Result;

use crate::trace::{
    backend::{ExecutionProof, ProofBackend},
    commitment::trace_root,
    trace::ExecutionTrace,
};

pub struct MockProofBackend;

impl ProofBackend for MockProofBackend {
    fn backend_id(&self) -> &'static str {
        "mock"
    }

    fn generate_proof(&self, trace: &ExecutionTrace) -> Result<ExecutionProof> {
        let root = trace_root(trace);
        Ok(ExecutionProof {
            backend_id: self.backend_id().to_string(),
            trace_root: root.clone(),
            proof_bytes: root.as_bytes().to_vec(),
        })
    }

    fn verify_proof(&self, proof: &ExecutionProof) -> Result<bool> {
        Ok(proof.backend_id == self.backend_id()
            && proof.proof_bytes == proof.trace_root.as_bytes())
    }
}
