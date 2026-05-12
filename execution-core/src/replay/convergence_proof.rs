use super::{proof_trace::ProofTrace, replay_proof::ReplayProof};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConvergenceProof {
    pub trace: ProofTrace,
    pub replay_proof: ReplayProof,
}
