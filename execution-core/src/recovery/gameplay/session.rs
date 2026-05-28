use crate::gameplay::GameplaySession;
pub fn restore_session(session_id: impl Into<String>) -> GameplaySession {
    GameplaySession::new(session_id)
}
