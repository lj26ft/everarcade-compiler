use everarcade_host::distributed_receipts::{
    execution_receipt::DistributedExecutionReceipt, receipt_aggregation::aggregate_receipt_root,
};

#[test]
fn receipt_roots_stable_for_same_inputs() {
    let receipts = vec![DistributedExecutionReceipt {
        receipt_root: [1; 32],
        task_root: [2; 32],
        package_root: [3; 32],
        operator_id: [4; 32],
    }];
    assert_eq!(
        aggregate_receipt_root(&receipts),
        aggregate_receipt_root(&receipts)
    );
}
