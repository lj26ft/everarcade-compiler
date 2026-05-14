use everarcade_host::replay_sync::replay_catchup::catchup;

#[test]
fn same_receipts_same_replay_root_invariant_modelled() {
    let result = catchup(10, true);
    assert!(result.replay_verified);
}
