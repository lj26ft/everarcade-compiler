use std::collections::BTreeMap;

use execution_core::state_engine::snapshot::StateSnapshot;
use execution_core::verifier::{
    ContractWasm, ReplayEngine, SyncObject, VerifierExecutionBundle, VerifierNode, VerifierSync,
};
use execution_core::{ExecutionNode, ExecutionPlan};

fn sample_plan() -> ExecutionPlan {
    ExecutionPlan {
        nodes: vec![ExecutionNode {
            id: "node-1".to_string(),
            action: "noop".to_string(),
            payload: serde_json::json!({"phase": "verifier"}),
            deps: vec![],
        }],
    }
}

fn sample_state() -> BTreeMap<String, String> {
    BTreeMap::from([("counter".to_string(), "42".to_string())])
}

#[test]
fn test_cross_verifier_replay() {
    let bundle = VerifierExecutionBundle {
        snapshot_state: sample_state(),
        plan: sample_plan(),
        contracts: vec![],
        expected_receipt: None,
    };

    let node_a = VerifierNode::new("A");
    let node_b = VerifierNode::new("B");

    let a = node_a.verify_bundle(&bundle);
    let b = node_b.verify_bundle(&bundle);

    assert_eq!(a.replay.receipt.receipt_hash, b.replay.receipt.receipt_hash);
}

#[test]
fn test_snapshot_sync() {
    let state = sample_state();
    let snapshot = StateSnapshot::new(state.clone(), None);

    let mut sync_a = VerifierSync::default();
    let mut sync_b = VerifierSync::default();

    sync_a.ingest(SyncObject::Snapshot(snapshot.clone()));
    sync_b.ingest(SyncObject::Snapshot(snapshot));

    assert_eq!(
        sync_a.snapshots[0].state_root,
        sync_b.snapshots[0].state_root
    );
}

#[test]
fn test_invalid_receipt_detection() {
    let mut tampered = ReplayEngine::replay(&VerifierExecutionBundle {
        snapshot_state: sample_state(),
        plan: sample_plan(),
        contracts: vec![],
        expected_receipt: None,
    })
    .receipt;
    tampered.receipt_hash = "tampered".to_string();

    let result = VerifierNode::new("challenger").verify_bundle(&VerifierExecutionBundle {
        snapshot_state: sample_state(),
        plan: sample_plan(),
        contracts: vec![],
        expected_receipt: Some(tampered),
    });

    assert!(result.challenge_triggered);
}

#[test]
fn test_execution_portability() {
    let portable = VerifierExecutionBundle {
        snapshot_state: sample_state(),
        plan: sample_plan(),
        contracts: vec![],
        expected_receipt: None,
    };

    let result = VerifierNode::new("independent").verify_bundle(&portable);
    assert!(!result.replay.receipt.receipt_hash.is_empty());
}

#[test]
fn test_contract_distribution() {
    let contract = ContractWasm {
        contract_id: "echo".to_string(),
        wasm_bytes: vec![0, 97, 115, 109],
    };

    let replay = VerifierNode::new("node-contract").verify_bundle(&VerifierExecutionBundle {
        snapshot_state: sample_state(),
        plan: sample_plan(),
        contracts: vec![contract],
        expected_receipt: None,
    });

    assert_eq!(replay.replay.contract_hashes.len(), 1);
    assert!(!replay.replay.contract_hashes[0].is_empty());
}
