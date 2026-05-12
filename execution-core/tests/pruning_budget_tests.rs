use execution_core::pruning::pruning_budget::within_pruning_budget;

#[test]
fn pruning_budget_is_checked() {
    assert!(within_pruning_budget(9, 10));
    assert!(!within_pruning_budget(11, 10));
}
