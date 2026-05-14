use execution_core::{
    receipt_runtime::execution_receipt::ExecutionReceipt, sync::validate_convergence, State,
};

#[test]
fn simulated_nodes_converge_to_same_replay_root() {
    let receipts = vec![ExecutionReceipt {
        receipt_id: "r1".into(),
        parent_receipt: None,
        execution_root: "e".into(),
        state_root: "s1".into(),
        graph_root: "g".into(),
        replay_root: "rr1".into(),
        timestamp_index: 0,
    }];
    let expected = execution_core::merkle::leaf_hash::leaf_hash(b"rr1");
    let res = validate_convergence(State::default(), None, &receipts, expected);
    assert!(res.converged);
}
