use anyhow::{bail, Result};

use crate::trace::{backend::{ExecutionProof, ProofBackend}, trace::ExecutionTrace};

pub struct Sp1Backend;

impl ProofBackend for Sp1Backend {
    fn backend_id(&self) -> &'static str { "sp1" }

    fn generate_proof(&self, _trace: &ExecutionTrace) -> Result<ExecutionProof> {
        bail!("SP1 backend is scaffolding only")
    }

    fn verify_proof(&self, _proof: &ExecutionProof) -> Result<bool> {
        bail!("SP1 backend is scaffolding only")
    }
}
