use super::peer_comparison::PeerContinuity;

pub fn select_highest_valid_replay(peers: &[PeerContinuity]) -> Option<&PeerContinuity> {
    peers
        .iter()
        .filter(|p| p.valid)
        .max_by_key(|p| p.continuity_height)
}
