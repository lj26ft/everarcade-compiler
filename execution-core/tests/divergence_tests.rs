use execution_core::divergence::{
    detect_divergence, hash_continuity_fork, register_divergence, verify_divergence,
    verify_divergence_policy, verify_divergence_proof, verify_divergence_window,
    verify_reconciliation_boundary, ContinuityFork, DivergencePolicy, DivergenceProof,
    DivergenceRegistry, DivergenceWindow, ReconciliationBoundary,
};

fn h(v: u8) -> [u8; 32] {
    [v; 32]
}

#[test]
fn test_continuity_fork_hash_stable() {
    let f = ContinuityFork {
        checkpoint_a: h(1),
        checkpoint_b: h(2),
        divergence_tick: 7,
        shared_ancestor: h(3),
    };
    assert_eq!(hash_continuity_fork(&f), hash_continuity_fork(&f));
}
#[test]
fn test_divergence_proof_valid() {
    let f = ContinuityFork {
        checkpoint_a: h(1),
        checkpoint_b: h(2),
        divergence_tick: 7,
        shared_ancestor: h(3),
    };
    let p = DivergenceProof {
        fork_hash: hash_continuity_fork(&f),
        finalized_checkpoint_a: h(1),
        finalized_checkpoint_b: h(2),
    };
    assert!(verify_divergence_proof(&p, &f));
}
#[test]
fn test_divergence_detection_valid() {
    let r = detect_divergence(h(1), h(2), h(8), h(9), Some(h(3)));
    assert!(r.divergence_detected);
}
#[test]
fn test_shared_ancestor_detected() {
    let r = detect_divergence(h(1), h(2), h(8), h(9), Some(h(3)));
    assert!(r.shared_ancestor_found);
}
#[test]
fn test_conflicting_finality_detected() {
    let f = ContinuityFork {
        checkpoint_a: h(1),
        checkpoint_b: h(2),
        divergence_tick: 7,
        shared_ancestor: h(3),
    };
    let p = DivergenceProof {
        fork_hash: hash_continuity_fork(&f),
        finalized_checkpoint_a: h(1),
        finalized_checkpoint_b: h(2),
    };
    let r = verify_divergence(&p, &f, h(10), h(11));
    assert!(r.conflicting_finality);
}
#[test]
fn test_duplicate_fork_rejected() {
    let mut reg = DivergenceRegistry::default();
    let f = ContinuityFork {
        checkpoint_a: h(1),
        checkpoint_b: h(2),
        divergence_tick: 7,
        shared_ancestor: h(3),
    };
    assert!(register_divergence(&mut reg, f.clone()).is_ok());
    assert!(register_divergence(&mut reg, f).is_err());
}
#[test]
fn test_divergence_registry_stable() {
    let mut reg = DivergenceRegistry::default();
    let f1 = ContinuityFork {
        checkpoint_a: h(1),
        checkpoint_b: h(2),
        divergence_tick: 7,
        shared_ancestor: h(3),
    };
    let f2 = ContinuityFork {
        checkpoint_a: h(4),
        checkpoint_b: h(5),
        divergence_tick: 8,
        shared_ancestor: h(6),
    };
    let k1 = register_divergence(&mut reg, f1).unwrap();
    let k2 = register_divergence(&mut reg, f2).unwrap();
    let keys: Vec<_> = reg.active_forks.keys().copied().collect();
    assert_eq!(keys, vec![k1.min(k2), k1.max(k2)]);
}
#[test]
fn test_divergence_window_valid() {
    assert!(verify_divergence_window(
        &DivergenceWindow {
            start_tick: 1,
            end_tick: 3
        },
        &[]
    )
    .is_ok());
}
#[test]
fn test_divergence_window_overlap_rejected() {
    let existing = [DivergenceWindow {
        start_tick: 1,
        end_tick: 3,
    }];
    assert!(verify_divergence_window(
        &DivergenceWindow {
            start_tick: 3,
            end_tick: 4
        },
        &existing
    )
    .is_err());
}
#[test]
fn test_divergence_verification_valid() {
    let f = ContinuityFork {
        checkpoint_a: h(1),
        checkpoint_b: h(2),
        divergence_tick: 7,
        shared_ancestor: h(3),
    };
    let p = DivergenceProof {
        fork_hash: hash_continuity_fork(&f),
        finalized_checkpoint_a: h(1),
        finalized_checkpoint_b: h(2),
    };
    let r = verify_divergence(&p, &f, h(10), h(11));
    assert!(r.valid);
}
#[test]
fn test_reconciliation_boundary_rejects_resolution() {
    assert!(verify_reconciliation_boundary(&ReconciliationBoundary {
        reconciliation_allowed: false
    }));
}
#[test]
fn test_divergence_policy_reject_conflicting_finality() {
    assert!(verify_divergence_policy(
        &DivergencePolicy {
            reject_conflicting_finality: true
        },
        true
    ));
    assert!(!verify_divergence_policy(
        &DivergencePolicy {
            reject_conflicting_finality: false
        },
        true
    ));
}
#[test]
fn test_divergence_replay_consistency() {
    let f = ContinuityFork {
        checkpoint_a: h(1),
        checkpoint_b: h(2),
        divergence_tick: 7,
        shared_ancestor: h(3),
    };
    let p = DivergenceProof {
        fork_hash: hash_continuity_fork(&f),
        finalized_checkpoint_a: h(1),
        finalized_checkpoint_b: h(2),
    };
    assert_eq!(
        verify_divergence(&p, &f, h(10), h(11)),
        verify_divergence(&p, &f, h(10), h(11))
    );
}
#[test]
fn test_divergence_deterministic() {
    let r1 = detect_divergence(h(1), h(2), h(8), h(9), Some(h(3)));
    let r2 = detect_divergence(h(1), h(2), h(8), h(9), Some(h(3)));
    assert_eq!(r1, r2);
}
