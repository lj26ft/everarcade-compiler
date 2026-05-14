use execution_core::domain::{domain::ExecutionDomain, domain_transition::transition_replay_root};

#[test]
fn domain_transition_updates_replay_root_only() {
    let domain = ExecutionDomain {
        domain_id: [1; 32],
        parent_domain: None,
        constitutional_root: [2; 32],
        governance_root: [3; 32],
        replay_root: [4; 32],
    };
    let next = transition_replay_root(&domain, [5; 32]);
    assert_eq!(next.replay_root, [5; 32]);
    assert_eq!(next.domain_id, domain.domain_id);
}
