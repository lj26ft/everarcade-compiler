use super::GovernanceContinuity;
pub fn governance_converged(peers: &[GovernanceContinuity]) -> bool {
    peers
        .windows(2)
        .all(|w| w[0].checkpoints.last() == w[1].checkpoints.last())
}
