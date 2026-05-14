use everarcade_host::replay_sync::receipt_exchange::validate_receipt_range;
#[test]
fn receipt_range_validation() {
    assert!(validate_receipt_range(1, 1));
    assert!(!validate_receipt_range(3, 2));
}
