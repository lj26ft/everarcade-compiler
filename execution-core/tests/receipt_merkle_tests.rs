use execution_core::{
    merkle::receipt_merkle::receipt_root, receipt_runtime::execution_receipt::ExecutionReceipt,
};

#[test]
fn receipt_root_deterministic() {
    let receipts = vec![ExecutionReceipt {
        receipt_id: "r1".into(),
        parent_receipt: None,
        execution_root: "e".into(),
        state_root: "s".into(),
        graph_root: "g".into(),
        replay_root: "r".into(),
        timestamp_index: 1,
    }];
    assert_eq!(receipt_root(&receipts), receipt_root(&receipts));
}
