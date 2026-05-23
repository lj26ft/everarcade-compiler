use execution_core::federation::sync_window::SyncWindow;
#[test]
fn sync_window_determinism() {
    let s = SyncWindow {
        replay_start: 0,
        replay_end: 10,
        checkpoint_start: 0,
        checkpoint_end: 5,
        settlement_start: 0,
        settlement_end: 2,
        archive_start: 0,
        archive_end: 1,
    };
    assert_eq!(s.canonical_hash().unwrap(), s.canonical_hash().unwrap());
}
