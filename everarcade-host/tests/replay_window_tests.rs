use everarcade_host::replay_sync::replay_window::ReplayWindow;

#[test]
fn same_replay_windows_same_replay_roots() {
    let window_a = ReplayWindow {
        start_receipt_root: [1; 32],
        end_receipt_root: [2; 32],
        receipt_count: 3,
        replay_root: [9; 32],
    };
    let window_b = window_a.clone();
    assert_eq!(window_a.replay_root, window_b.replay_root);
}
