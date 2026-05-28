use super::replay_cursor::ReplayCursor;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayReconnectState {
    pub last_acknowledged: ReplayCursor,
    pub peer_equivalent: bool,
}
impl ReplayReconnectState {
    pub fn resume(cursor: ReplayCursor, expected_root: &str) -> Result<Self, String> {
        if cursor.continuity_root != expected_root {
            return Err("replay_fork_injection_rejected".into());
        }
        Ok(Self {
            last_acknowledged: cursor,
            peer_equivalent: true,
        })
    }
}
