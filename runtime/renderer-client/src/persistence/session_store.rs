use std::collections::BTreeMap;

use super::artifact::RenderProjectionArtifact;

#[derive(Debug, Clone, Default)]
pub struct ProjectionSessionStore {
    sessions: BTreeMap<String, Vec<RenderProjectionArtifact>>,
}

impl ProjectionSessionStore {
    pub fn persist_projection_session(
        &mut self,
        session_id: &str,
        frames: Vec<RenderProjectionArtifact>,
    ) {
        self.sessions.insert(session_id.to_string(), frames);
    }

    pub fn restore_projection_session(
        &self,
        session_id: &str,
    ) -> Option<Vec<RenderProjectionArtifact>> {
        self.sessions.get(session_id).cloned()
    }
}
