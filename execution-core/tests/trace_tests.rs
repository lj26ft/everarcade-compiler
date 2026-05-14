use execution_core::trace::{
    backend::ProofBackend,
    backend_mock::MockProofBackend,
    commitment::{replay_commitment, trace_root},
    compatibility::{validate_compatibility, TraceSchemaCompatibility},
    merkle_trace::merkle_trace_root,
    replay::{replay_matches, replay_trace},
    serialization::serialize_trace,
    trace::{ExecutionTrace, TraceNode},
    transition::{TraceOperation, TraceTransition},
};

fn sample_trace() -> ExecutionTrace {
    ExecutionTrace {
        trace_id: "trace-1".into(),
        epoch_id: 7,
        execution_root: "exec-root".into(),
        state_root_before: "state-before".into(),
        state_root_after: "state-after".into(),
        nodes: vec![TraceNode {
            node_id: "n1".into(),
            op_code: "call".into(),
            input_hash: "i".into(),
            output_hash: "o".into(),
            index: 0,
        }],
        transitions: vec![TraceTransition {
            transition_id: "t1".into(),
            node_id: "n1".into(),
            operation: TraceOperation::NodeExecution,
            before: "a".into(),
            after: "b".into(),
            memory_delta: 4,
            fuel_delta: 2,
            index: 0,
        }],
        fuel_used: 2,
        memory_used: 4,
    }
}

#[test]
fn test_trace_determinism() {
    let trace = sample_trace();
    assert_eq!(replay_trace(&trace), replay_trace(&trace));
}

#[test]
fn test_trace_serialization_stability() {
    let trace = sample_trace();
    assert_eq!(serialize_trace(&trace), serialize_trace(&trace));
}

#[test]
fn test_trace_commitment_stability() {
    let trace = sample_trace();
    assert_eq!(trace_root(&trace), trace_root(&trace));
    assert_eq!(replay_commitment(&trace), replay_commitment(&trace));
}

#[test]
fn test_trace_replay_reconstruction() {
    let trace = sample_trace();
    let replayed = replay_trace(&trace);
    assert!(replay_matches(&trace, &replayed));
}

#[test]
fn test_backend_abstraction_consistency() {
    let trace = sample_trace();
    let backend = MockProofBackend;
    let proof = backend.generate_proof(&trace).unwrap();
    assert!(backend.verify_proof(&proof).unwrap());
    assert_eq!(proof.trace_root, trace_root(&trace));
}

#[test]
fn test_epoch_aware_trace_compatibility() {
    let trace = sample_trace();
    let backend = MockProofBackend;
    let proof = backend.generate_proof(&trace).unwrap();
    let ok = validate_compatibility(&trace, &proof, 7);
    assert_eq!(ok.trace_schema, TraceSchemaCompatibility::Compatible);
    let bad = validate_compatibility(&trace, &proof, 8);
    assert_eq!(bad.trace_schema, TraceSchemaCompatibility::EpochMismatch);
}

#[test]
fn test_merkle_trace_determinism() {
    let trace = sample_trace();
    assert_eq!(merkle_trace_root(&trace), merkle_trace_root(&trace));
}

#[test]
fn test_proof_backend_independence() {
    let trace = sample_trace();
    let a = MockProofBackend;
    let b = MockProofBackend;
    let pa = a.generate_proof(&trace).unwrap();
    let pb = b.generate_proof(&trace).unwrap();
    assert_eq!(pa.trace_root, pb.trace_root);
    assert!(a.verify_proof(&pa).unwrap());
    assert!(b.verify_proof(&pb).unwrap());
}
