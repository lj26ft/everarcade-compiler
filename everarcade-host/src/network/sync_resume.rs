use super::sync_session::SyncSession;

pub fn resume_from_last_window(session: &SyncSession) -> u64 {
    session.last_validated_window
}
