use execution_core::execution::starvation::is_starving;

#[test]
fn starvation_threshold_enforced() {
    assert!(is_starving(5, 5));
    assert!(!is_starving(4, 5));
}
