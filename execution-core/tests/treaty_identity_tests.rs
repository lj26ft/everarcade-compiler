use execution_core::treaty::{treaty::ExecutionTreaty, treaty_identity::treaty_identity_hash};

#[test]
fn treaty_identity_is_deterministic() {
    let treaty = ExecutionTreaty {
        treaty_id: [1; 32],
        participating_domains: vec![[2; 32], [3; 32]],
        constitutional_scope_root: [4; 32],
        capability_scope_root: [5; 32],
        arbitration_root: [6; 32],
    };
    assert_eq!(treaty_identity_hash(&treaty), treaty_identity_hash(&treaty));
}
