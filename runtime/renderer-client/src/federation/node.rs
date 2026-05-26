#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectionObserverClient;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectionObserverSession {
    pub session_id: String,
    pub continuity_root: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ProjectionObserverReplayCursor {
    pub sequence: u64,
}
