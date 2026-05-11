use super::voting::VerifierVote;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DisputeOutcome {
    ChallengerWins,
    DefenderWins,
}

pub fn resolve_dispute(verifier_a: &VerifierVote, verifier_b: &VerifierVote) -> DisputeOutcome {
    if verifier_a.receipt_hash <= verifier_b.receipt_hash {
        DisputeOutcome::ChallengerWins
    } else {
        DisputeOutcome::DefenderWins
    }
}
