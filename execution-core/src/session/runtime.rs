use crate::gameplay::{GameplayRuntime, GameplaySession};

#[derive(Clone, Debug, Default)]
pub struct SessionRuntime {
    pub sessions: Vec<GameplaySession>,
}

impl SessionRuntime {
    pub fn create_session(&mut self, session_id: impl Into<String>) -> GameplayRuntime {
        let session = GameplaySession::new(session_id);
        self.sessions.push(session.clone());
        GameplayRuntime::new(session)
    }
}
