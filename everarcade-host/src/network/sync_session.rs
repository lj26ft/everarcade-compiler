#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SyncSession {
    pub session_id: String,
    pub last_validated_window: u64,
}
