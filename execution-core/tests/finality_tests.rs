use std::collections::BTreeSet;

use execution_core::{
    federation::node::FederationNodeId,
    finality::{
        ack::{verify_ack, FinalityAcknowledgment},
        checkpoint::{hash_finalized_checkpoint, FinalizedCheckpoint},
        finalize::finalize_checkpoint,
        policy::{verify_finality_policy, FinalityPolicy},
        proof::{verify_finality_proof, FinalityProof},
        quorum::{verify_quorum, FinalityQuorum},
        registry::{update_finality_registry, FinalityRegistry},
        verification::verify_finalization,
        window::{verify_finalization_window, FinalizationWindow},
    },
};

fn h(v: u8) -> [u8; 32] {
    [v; 32]
}
fn n(v: u8) -> FederationNodeId {
    FederationNodeId::new([v; 32])
}

fn checkpoint() -> FinalizedCheckpoint {
    FinalizedCheckpoint {
        checkpoint_root: h(1),
        execution_id: h(2),
        finalized_tick: 10,
        acknowledged_observers: BTreeSet::from([n(1), n(2)]),
    }
}

#[test]
fn test_finalized_checkpoint_hash_stable() {
    let c = checkpoint();
    assert_eq!(hash_finalized_checkpoint(&c), hash_finalized_checkpoint(&c));
}
#[test]
fn test_acknowledgment_valid() {
    let a = FinalityAcknowledgment {
        observer: n(1),
        checkpoint_root: h(1),
        execution_id: h(2),
    };
    assert!(verify_ack(&a, &[n(1), n(2)], h(1), h(2)).is_ok());
}
#[test]
fn test_duplicate_ack_rejected() {
    let q = FinalityQuorum {
        required_observers: 2,
    };
    let a = FinalityAcknowledgment {
        observer: n(1),
        checkpoint_root: h(1),
        execution_id: h(2),
    };
    assert!(verify_quorum(&q, &[a.clone(), a]).is_err());
}
#[test]
fn test_finalization_window_valid() {
    assert!(verify_finalization_window(
        &FinalizationWindow {
            start_tick: 11,
            end_tick: 20
        },
        Some(&FinalizationWindow {
            start_tick: 0,
            end_tick: 10
        })
    )
    .is_ok());
}
#[test]
fn test_finalization_window_overlap_rejected() {
    assert!(verify_finalization_window(
        &FinalizationWindow {
            start_tick: 10,
            end_tick: 20
        },
        Some(&FinalizationWindow {
            start_tick: 0,
            end_tick: 10
        })
    )
    .is_err());
}
#[test]
fn test_quorum_reached() {
    let q = FinalityQuorum {
        required_observers: 2,
    };
    let a1 = FinalityAcknowledgment {
        observer: n(1),
        checkpoint_root: h(1),
        execution_id: h(2),
    };
    let a2 = FinalityAcknowledgment {
        observer: n(2),
        checkpoint_root: h(1),
        execution_id: h(2),
    };
    assert!(verify_quorum(&q, &[a1, a2]).is_ok());
}
#[test]
fn test_quorum_failure() {
    let q = FinalityQuorum {
        required_observers: 2,
    };
    let a1 = FinalityAcknowledgment {
        observer: n(1),
        checkpoint_root: h(1),
        execution_id: h(2),
    };
    assert!(verify_quorum(&q, &[a1]).is_err());
}
#[test]
fn test_checkpoint_finalization_valid() {
    let c = checkpoint();
    let q = FinalityQuorum {
        required_observers: 2,
    };
    let w = FinalizationWindow {
        start_tick: 11,
        end_tick: 20,
    };
    let p = FinalizationWindow {
        start_tick: 0,
        end_tick: 10,
    };
    let a1 = FinalityAcknowledgment {
        observer: n(1),
        checkpoint_root: h(1),
        execution_id: h(2),
    };
    let a2 = FinalityAcknowledgment {
        observer: n(2),
        checkpoint_root: h(1),
        execution_id: h(2),
    };
    assert!(
        finalize_checkpoint(&c, &[a1, a2], &q, &w, Some(&p))
            .unwrap()
            .finalized
    );
}
#[test]
fn test_finalized_rollback_rejected() {
    let c = FinalizedCheckpoint {
        finalized_tick: 5,
        ..checkpoint()
    };
    let r = FinalityRegistry {
        latest_finalized_checkpoint: h(9),
        latest_finalized_tick: 10,
    };
    assert!(update_finality_registry(&r, &c).is_err());
}
#[test]
fn test_finality_registry_monotonic() {
    let c = checkpoint();
    let r = FinalityRegistry {
        latest_finalized_checkpoint: h(0),
        latest_finalized_tick: 9,
    };
    assert_eq!(
        update_finality_registry(&r, &c)
            .unwrap()
            .latest_finalized_tick,
        10
    );
}
#[test]
fn test_finality_proof_valid() {
    let c = checkpoint();
    let p = FinalityProof {
        finalized_checkpoint_hash: hash_finalized_checkpoint(&c),
        finalized_tick: 10,
    };
    assert!(verify_finality_proof(&p, &c).is_ok());
}
#[test]
fn test_finality_policy_require_quorum() {
    let pol = FinalityPolicy {
        require_quorum: true,
    };
    let report = execution_core::finality::finalize::FinalizationReport {
        finalized: true,
        quorum_reached: true,
    };
    assert!(verify_finality_policy(&pol, &report).is_ok());
}
#[test]
fn test_finality_replay_consistency() {
    let c = checkpoint();
    assert_eq!(hash_finalized_checkpoint(&c), hash_finalized_checkpoint(&c));
}
#[test]
fn test_finalization_deterministic() {
    let c = checkpoint();
    let q = FinalityQuorum {
        required_observers: 2,
    };
    let r = FinalityRegistry {
        latest_finalized_checkpoint: h(0),
        latest_finalized_tick: 0,
    };
    let a1 = FinalityAcknowledgment {
        observer: n(1),
        checkpoint_root: h(1),
        execution_id: h(2),
    };
    let a2 = FinalityAcknowledgment {
        observer: n(2),
        checkpoint_root: h(1),
        execution_id: h(2),
    };
    let x = verify_finalization(&c, &[a1.clone(), a2.clone()], &q, &r);
    let y = verify_finalization(&c, &[a1, a2], &q, &r);
    assert_eq!(x, y);
}
