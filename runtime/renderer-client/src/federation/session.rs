#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectionFederationSession {
    pub session_id: String,
    pub node_id: String,
    pub continuity_root: String,
}
