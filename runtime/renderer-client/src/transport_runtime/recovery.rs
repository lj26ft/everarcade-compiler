use super::stream::ReplayTransportCursor;

#[derive(Debug, Clone, Default)]
pub struct ReplayCatchupRuntime;

#[derive(Debug, Clone, Default)]
pub struct ReplayCatchupWindow {
    pub resume_from_sequence: u64,
    pub pending_until_sequence: u64,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ReplayResumeState {
    pub cursor: ReplayTransportCursor,
}

impl ReplayCatchupRuntime {
    pub fn resume_state(cursor: &ReplayTransportCursor) -> ReplayResumeState {
        ReplayResumeState { cursor: cursor.clone() }
    }
}
