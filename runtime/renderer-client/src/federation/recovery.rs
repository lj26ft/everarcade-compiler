#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectionReplayRecoveryState {
    pub session_id: String,
    pub last_sequence: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectionReplayResumePoint {
    pub stream_id: String,
    pub sequence: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectionReplayCatchupWindow {
    pub from_sequence: u64,
    pub to_sequence: u64,
}
