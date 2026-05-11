use execution_core::proof::*;

fn sample_trace() -> ExecutionTrace {
    ExecutionTrace {
        epoch_id: 7,
        execution_root: "root-abc".to_string(),
        snapshot_hash: "snapshot-xyz".to_string(),
        nodes: vec![TraceNode {
            node_id: "n1".into(),
            contract_hash: "c1".into(),
            fuel_used: 12,
            memory_used: 128,
        }],
        transitions: vec![TraceTransition {
            key: "counter".into(),
            old_value: Some("1".into()),
            new_value: "2".into(),
        }],
    }
}

#[test]
fn test_execution_trace_determinism() {
    assert_eq!(sample_trace().digest(), sample_trace().digest());
}

#[test]
fn test_proof_receipt_binding() {
    let trace = sample_trace();
    let proof = deterministic_prove(&trace, "receipt-1", "placeholder");
    assert!(verify_receipt_binding(&proof, "receipt-1"));
}

#[test]
fn test_proof_serialization_stability() {
    let proof = deterministic_prove(&sample_trace(), "receipt-1", "placeholder");
    let a = bincode::serialize(&proof).unwrap();
    let b = bincode::serialize(&proof).unwrap();
    assert_eq!(a, b);
}

#[test]
fn test_execution_commitment_stability() {
    let trace = sample_trace();
    let c1 = trace_commitment(&trace);
    let c2 = trace_commitment(&trace);
    assert_eq!(execution_commitment(&trace.execution_root, &c1), execution_commitment(&trace.execution_root, &c2));
}

#[test]
fn test_aggregate_proof_consistency() {
    let t = sample_trace();
    let p1 = deterministic_prove(&t, "r1", "placeholder");
    let p2 = deterministic_prove(&t, "r2", "placeholder");
    assert_eq!(aggregate_proofs(&[p1.clone(), p2.clone()]), aggregate_proofs(&[p1, p2]));
}

#[test]
fn test_recursive_proof_lineage() {
    let p = deterministic_prove(&sample_trace(), "r1", "placeholder");
    let agg = aggregate_proofs(&[p]);
    let l1 = compose_lineage(&agg, "parent");
    let l2 = compose_lineage(&agg, "parent");
    assert_eq!(l1, l2);
}

#[test]
fn test_epoch_aware_proof_validation() {
    let proof = deterministic_prove(&sample_trace(), "r1", "placeholder");
    assert!(verify_epoch(&proof, 7));
    assert!(!verify_epoch(&proof, 8));
}

#[test]
fn test_proof_transport_integrity() {
    let proof = deterministic_prove(&sample_trace(), "r1", "placeholder");
    let chunks = chunk_proof(&proof.proof_bytes, 8);
    let reconstructed = reconstruct_proof(&chunks);
    assert_eq!(proof.proof_bytes, reconstructed);
}

#[test]
fn test_replay_fallback_consistency() {
    let trace = sample_trace();
    let proof = deterministic_prove(&trace, "receipt-1", "placeholder");
    assert!(verify_proof_integrity(&proof));
    assert!(verify_execution_root(&proof, &trace.execution_root));
}
