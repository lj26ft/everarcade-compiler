#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SessionContinuity {
    pub session_id: String,
    pub continuity_root: String,
    pub replay_tip: u64,
}
