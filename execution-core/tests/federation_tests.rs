use execution_core::federation::{
    collective_execution::CollectiveExecutionPlan,
    constitutional_rules::ConstitutionalRules,
    federation::Federation,
    governance::{GovernanceAction, GovernanceState},
    quorum::QuorumRule,
    treaty::Treaty,
};

#[test]
fn test_federation_identity_stability() {
    let a = Federation::new(vec!["e2".into(), "e1".into()]);
    let b = Federation::new(vec!["e1".into(), "e2".into()]);
    assert_eq!(a.federation_id, b.federation_id);
}

#[test]
fn test_deterministic_governance() {
    let actions = vec![GovernanceAction {
        actor: "a".into(),
        action: "upgrade".into(),
    }];
    assert_eq!(
        GovernanceState::apply(&actions),
        GovernanceState::apply(&actions)
    );
}

#[test]
fn test_treaty_continuity() {
    let genesis = Treaty::genesis(b"alpha");
    let v2 = genesis.evolve(b"beta");
    assert_eq!(v2.previous_treaty, Some(genesis.treaty_id));
}

#[test]
fn test_collective_execution() {
    let a = CollectiveExecutionPlan::new(vec!["b".into(), "a".into()]);
    let b = CollectiveExecutionPlan::new(vec!["a".into(), "b".into()]);
    assert_eq!(a.execution_root, b.execution_root);
}

#[test]
fn test_quorum_replay_consistency() {
    let q = QuorumRule {
        numerator: 2,
        denominator: 3,
    };
    assert!(q.reached(4, 6));
    assert!(!q.reached(3, 6));
}

#[test]
fn test_institutional_migration() {
    let a = Federation::new(vec!["e1".into()]);
    let b = Federation::new(vec!["e1".into()]);
    assert_eq!(a.continuity_root, b.continuity_root);
}

#[test]
fn test_constitutional_continuity() {
    let c1 = ConstitutionalRules {
        constitution_hash: "genesis".into(),
        version: 1,
    };
    let c2 = c1.upgrade(b"patch");
    assert_eq!(c2.version, 2);
}

#[test]
fn test_civilizational_continuity() {
    let c1 = ConstitutionalRules {
        constitution_hash: "g".into(),
        version: 1,
    };
    let c2 = c1.upgrade(b"epoch-2");
    let c3 = c2.upgrade(b"epoch-3");
    assert_eq!(c3.version, 3);
}
