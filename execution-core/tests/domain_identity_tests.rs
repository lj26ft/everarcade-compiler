use execution_core::domain::{domain::ExecutionDomain, domain_identity::domain_identity};

#[test]
fn domain_identity_is_stable() {
    let domain = ExecutionDomain {
        domain_id: [1; 32],
        parent_domain: None,
        constitutional_root: [2; 32],
        governance_root: [3; 32],
        replay_root: [4; 32],
    };
    assert_eq!(domain_identity(&domain), [1; 32]);
}
