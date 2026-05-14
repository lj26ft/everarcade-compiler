use everarcade_host::convergence::{
    anchor_exchange::AnchorExchange, checkpoint_exchange::CheckpointExchange,
    convergence_engine::converge_roots, receipt_exchange::ReceiptExchange,
};

#[test]
fn same_artifacts_same_replay_roots() {
    let receipts = ReceiptExchange {
        receipt_ids: vec!["r1".into()],
    };
    let checkpoints = CheckpointExchange {
        checkpoint_roots: vec!["c1".into()],
    };
    let anchors = AnchorExchange {
        anchor_ids: vec!["a1".into()],
    };
    assert!(converge_roots(&receipts, &checkpoints, &anchors).is_ok());
}
