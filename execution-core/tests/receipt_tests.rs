use execution_core::receipt_canonical::CanonicalExecutionReceipt;

#[test]
fn canonical_receipt_hash_is_stable() {
    let receipt = CanonicalExecutionReceipt {
        execution_id: [1; 32],
        module_hash: [2; 32],
        input_hash: [3; 32],
        fuel_consumed: 123,
        state_root_before: [4; 32],
        state_root_after: [5; 32],
        state_diff_hash: [6; 32],
        replay_hash: [7; 32],
        exit_code: 0,
    };

    assert_eq!(receipt.canonical_bytes(), receipt.canonical_bytes());
    assert_eq!(receipt.canonical_hash(), receipt.canonical_hash());
}
