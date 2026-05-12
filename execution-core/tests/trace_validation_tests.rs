use execution_core::replay::replay_trace::validate_step_link;
use execution_core::replay::TraceStep;

#[test]
fn trace_step_link_validates_index_and_root_continuity() {
    let a = TraceStep { logical_index: 0, receipt_hash: "r0".into(), parent_receipt_hash: None, prior_state_root: "g".into(), transition_root: "t0".into(), next_state_root: "n0".into(), replay_root: "p0".into() };
    let b = TraceStep { logical_index: 1, receipt_hash: "r1".into(), parent_receipt_hash: Some("r0".into()), prior_state_root: "n0".into(), transition_root: "t1".into(), next_state_root: "n1".into(), replay_root: "p1".into() };
    assert!(validate_step_link(None, &a));
    assert!(validate_step_link(Some(&a), &b));
}
