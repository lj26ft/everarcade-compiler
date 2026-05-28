use super::replay_cursor::ReplayCursor;
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ReplayCatchupPlan {
    pub from: ReplayCursor,
    pub requested_windows: Vec<u64>,
}
impl ReplayCatchupPlan {
    pub fn from_cursor(cursor: ReplayCursor) -> Self {
        Self {
            from: cursor,
            requested_windows: vec![0],
        }
    }
}
