use execution_core::governance::*;

fn h(v: u8) -> [u8; 32] {
    [v; 32]
}

#[test]
fn test_governance_proposal_determinism() {
    let p1 = create_governance_proposal(1, "increase-world-cap".into(), Some(h(1)));
    let p2 = create_governance_proposal(1, "increase-world-cap".into(), Some(h(1)));
    assert_eq!(p1, p2);
}

#[test]
fn test_duplicate_vote_rejection() {
    let p = create_governance_proposal(1, "p".into(), Some(h(1)));
    let v1 = submit_governance_vote(&p, h(9), true, 1, &[]).unwrap();
    let err = submit_governance_vote(&p, h(9), true, 1, &[v1]).unwrap_err();
    assert_eq!(err, GovernanceError::DuplicateVote);
}

#[test]
fn test_governance_replay_convergence() {
    let p0 = create_governance_proposal(0, "genesis".into(), None);
    let p1 = create_governance_proposal(1, "policy".into(), Some(p0.id));
    let v = submit_governance_vote(&p1, h(2), true, 1, &[]).unwrap();
    let cp = GovernanceCheckpoint {
        epoch: 1,
        proposal_root: h(3),
        vote_root: h(4),
        policy_root: h(5),
        authority_root: h(6),
    };
    let pol = apply_governance_policy(1, true, vec!["runtime:all".into()], &cp);
    let a0 = assign_runtime_authority(h(1), 0, vec!["orchestrate".into()]);
    let a1 = transfer_runtime_authority(&a0, h(2), vec!["orchestrate".into(), "migrate".into()]);
    let continuity = GovernanceContinuity {
        proposals: vec![p0, p1],
        votes: vec![v],
        policies: vec![pol],
        authorities: vec![a0, a1],
        checkpoints: vec![cp],
    };
    assert!(verify_governance_integrity(&continuity).is_ok());
    assert!(verify_governance_replay(&continuity).is_ok());
}
