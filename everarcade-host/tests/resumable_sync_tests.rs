use everarcade_host::network::{sync_resume::resume_from_last_window, sync_session::SyncSession};

#[test]
fn interrupted_sync_resumes_from_last_validated_window() {
    let session = SyncSession {
        session_id: "s-1".into(),
        last_validated_window: 42,
    };
    assert_eq!(resume_from_last_window(&session), 42);
}
