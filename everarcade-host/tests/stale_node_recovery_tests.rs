use everarcade_host::partition_recovery::recovery_plan::RecoveryPlan;

#[test]
fn stale_node_recovery_plan_tracks_missing_receipts() {
    let plan = RecoveryPlan {
        latest_checkpoint_root: [7; 32],
        missing_receipts: 5,
        requires_checkpoint_import: true,
    };
    assert_eq!(plan.missing_receipts, 5);
}
