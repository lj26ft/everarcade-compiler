use execution_core::{
    federation::node::FederationNodeId,
    reconciliation::{
        hash_reconciliation_descriptor, register_quarantined_fork, verify_quarantine,
        verify_reconciliation, verify_reconciliation_boundary, verify_reconciliation_policy,
        verify_reconciliation_proof, verify_reconciliation_request, QuarantinedFork,
        ReconciliationBoundary, ReconciliationDescriptor, ReconciliationPolicy,
        ReconciliationProof, ReconciliationRegistry, ReconciliationRequest,
    },
};

fn h(v: u8) -> [u8; 32] {
    [v; 32]
}

fn descriptor() -> ReconciliationDescriptor {
    ReconciliationDescriptor {
        fork_hash: h(9),
        checkpoint_a: h(1),
        checkpoint_b: h(2),
        reconciliation_allowed: false,
    }
}

#[test]
fn test_reconciliation_descriptor_hash_stable() {
    let d = descriptor();
    assert_eq!(
        hash_reconciliation_descriptor(&d),
        hash_reconciliation_descriptor(&d)
    );
}

#[test]
fn test_reconciliation_request_valid() {
    let mut registry = ReconciliationRegistry::default();
    let d = descriptor();
    register_quarantined_fork(&mut registry, d.clone()).unwrap();
    let req = ReconciliationRequest {
        fork_hash: d.fork_hash,
        requested_by: FederationNodeId::new(h(3)),
    };
    assert!(verify_reconciliation_request(&req, &registry));
}

#[test]
fn test_duplicate_quarantine_rejected() {
    let mut registry = ReconciliationRegistry::default();
    let d = descriptor();
    assert!(register_quarantined_fork(&mut registry, d.clone()).is_ok());
    assert!(register_quarantined_fork(&mut registry, d).is_err());
}

#[test]
fn test_registry_stable() {
    let mut registry = ReconciliationRegistry::default();
    let mut d1 = descriptor();
    d1.fork_hash = h(1);
    let mut d2 = descriptor();
    d2.fork_hash = h(2);
    let k1 = register_quarantined_fork(&mut registry, d1).unwrap();
    let k2 = register_quarantined_fork(&mut registry, d2).unwrap();
    let keys: Vec<_> = registry.quarantined_forks.keys().copied().collect();
    assert_eq!(keys, vec![k1, k2]);
}

#[test]
fn test_reconciliation_verification_valid() {
    let mut registry = ReconciliationRegistry::default();
    let d = descriptor();
    register_quarantined_fork(&mut registry, d.clone()).unwrap();
    let report = verify_reconciliation(d.fork_hash, &registry);
    assert!(report.quarantined);
    assert!(report.reconciliation_prohibited);
}

#[test]
fn test_reconciliation_prohibited() {
    let mut registry = ReconciliationRegistry::default();
    let mut d = descriptor();
    d.reconciliation_allowed = true;
    register_quarantined_fork(&mut registry, d.clone()).unwrap();
    let report = verify_reconciliation(d.fork_hash, &registry);
    assert!(!report.reconciliation_prohibited);
}

#[test]
fn test_reconciliation_boundary_disabled() {
    assert!(verify_reconciliation_boundary(&ReconciliationBoundary {
        automatic_reconciliation_disabled: true
    }));
}

#[test]
fn test_reconciliation_policy_requires_quarantine() {
    let policy = ReconciliationPolicy {
        quarantine_required: true,
    };
    assert!(verify_reconciliation_policy(&policy, true));
    assert!(!verify_reconciliation_policy(&policy, false));
}

#[test]
fn test_quarantine_valid() {
    let mut registry = ReconciliationRegistry::default();
    let d = descriptor();
    register_quarantined_fork(&mut registry, d.clone()).unwrap();
    let q = QuarantinedFork {
        fork_hash: d.fork_hash,
        quarantined: true,
    };
    assert!(verify_quarantine(&q, &registry));
}

#[test]
fn test_quarantine_replay_consistency() {
    let mut registry = ReconciliationRegistry::default();
    let d = descriptor();
    register_quarantined_fork(&mut registry, d.clone()).unwrap();
    let q = QuarantinedFork {
        fork_hash: d.fork_hash,
        quarantined: true,
    };
    assert_eq!(
        verify_quarantine(&q, &registry),
        verify_quarantine(&q, &registry)
    );
}

#[test]
fn test_reconciliation_deterministic() {
    let mut registry = ReconciliationRegistry::default();
    let d = descriptor();
    register_quarantined_fork(&mut registry, d.clone()).unwrap();
    let p = ReconciliationProof {
        fork_hash: d.fork_hash,
        quarantine_hash: hash_reconciliation_descriptor(&d),
    };
    assert!(verify_reconciliation_proof(&p, &registry));
    assert_eq!(
        verify_reconciliation(d.fork_hash, &registry),
        verify_reconciliation(d.fork_hash, &registry)
    );
}
