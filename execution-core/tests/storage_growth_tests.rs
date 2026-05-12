use execution_core::pruning::storage_growth::storage_growth;

#[test]
fn storage_growth_is_non_negative() {
    assert_eq!(storage_growth(100, 90), 0);
    assert_eq!(storage_growth(100, 105), 5);
}
