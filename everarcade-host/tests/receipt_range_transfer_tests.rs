use everarcade_host::replay_sync::receipt_range_validation::validate_ordered_and_contiguous;

#[test]
fn gaps_fail() {
    assert!(!validate_ordered_and_contiguous(&[1, 3, 4]));
}
