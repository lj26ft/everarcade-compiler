use anyhow::{bail, Result};

use crate::trace::{
    backend::{ExecutionProof, ProofBackend},
    trace::ExecutionTrace,
};

pub struct Risc0Backend;

impl ProofBackend for Risc0Backend {
    fn backend_id(&self) -> &'static str {
        "risc0"
    }

    fn generate_proof(&self, _trace: &ExecutionTrace) -> Result<ExecutionProof> {
        bail!("RISC Zero backend is scaffolding only")
    }

    fn verify_proof(&self, _proof: &ExecutionProof) -> Result<bool> {
        bail!("RISC Zero backend is scaffolding only")
    }
}
