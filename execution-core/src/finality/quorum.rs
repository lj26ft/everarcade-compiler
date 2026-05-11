#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuorumConfig {
    pub minimum_verifiers: u64,
    pub threshold_numerator: u64,
    pub threshold_denominator: u64,
    pub challenge_window_blocks: u64,
}

impl QuorumConfig {
    pub fn replay_quorum_reached(&self, agreeing: u64, total: u64) -> bool {
        if total < self.minimum_verifiers || self.threshold_denominator == 0 {
            return false;
        }
        agreeing.saturating_mul(self.threshold_denominator)
            >= total.saturating_mul(self.threshold_numerator)
    }
}

impl Default for QuorumConfig {
    fn default() -> Self {
        Self {
            minimum_verifiers: 3,
            threshold_numerator: 2,
            threshold_denominator: 3,
            challenge_window_blocks: 10,
        }
    }
}
