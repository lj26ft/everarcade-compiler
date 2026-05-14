use super::{execution_session::ExecutionSession, session_root::compute_session_id};

pub fn validate_execution_session(session: &ExecutionSession) -> bool {
    compute_session_id(
        session.parent_session,
        session.replay_root,
        session.convergence_root,
    ) == session.session_id
}
