use crate::{
    checkpoint::{
        checkpoint_snapshot::CheckpointSnapshot, checkpoint_validation::validate_checkpoint,
    },
    merkle::{inclusion_proof::InclusionProof, proof_validation::validate_proof, Hash},
    replay::replay_proof::ReplayProof,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StateProof {
    pub root: Hash,
    pub leaf: Hash,
    pub proof: InclusionProof,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReceiptProof {
    pub root: Hash,
    pub leaf: Hash,
    pub proof: InclusionProof,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProofExchange {
    pub state_proofs: Vec<StateProof>,
    pub receipt_proofs: Vec<ReceiptProof>,
    pub replay_proof: Option<ReplayProof>,
    pub checkpoint: Option<CheckpointSnapshot>,
}

pub fn validate_proof_exchange(exchange: &ProofExchange) -> bool {
    let state_ok = exchange
        .state_proofs
        .iter()
        .all(|p| validate_proof(p.root, p.leaf, &p.proof));
    let receipt_ok = exchange
        .receipt_proofs
        .iter()
        .all(|p| validate_proof(p.root, p.leaf, &p.proof));
    let checkpoint_ok = exchange.checkpoint.as_ref().is_none_or(validate_checkpoint);
    state_ok && receipt_ok && checkpoint_ok
}
