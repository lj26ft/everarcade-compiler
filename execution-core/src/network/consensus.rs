#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConsensusVote {
    pub verifier_id: String,
    pub receipt_hash: String,
}

#[derive(Debug, Default)]
pub struct ReplayConsensus;

impl ReplayConsensus {
    pub fn agrees(votes: &[ConsensusVote], quorum: usize) -> bool {
        if votes.is_empty() || quorum == 0 {
            return false;
        }
        let candidate = &votes[0].receipt_hash;
        votes
            .iter()
            .filter(|v| v.receipt_hash == *candidate)
            .count()
            >= quorum
    }
}
