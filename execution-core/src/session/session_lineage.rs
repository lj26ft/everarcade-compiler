use super::execution_session::ExecutionSession;

pub fn session_continuity(parent: &ExecutionSession, child: &ExecutionSession) -> bool {
    child.parent_session == Some(parent.session_id)
}
