#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PeerContinuity {
    pub peer_id: String,
    pub continuity_height: u64,
    pub valid: bool,
}
