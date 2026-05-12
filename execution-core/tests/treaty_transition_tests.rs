use execution_core::treaty::{treaty::ExecutionTreaty, treaty_transition::amend_treaty};
#[test]
fn treaty_amendment_updates_scope() {
 let treaty=ExecutionTreaty{treaty_id:[1;32],participating_domains:vec![[9;32]],constitutional_scope_root:[2;32],capability_scope_root:[3;32],arbitration_root:[4;32]};
 let next=amend_treaty(&treaty,[8;32]);
 assert_eq!(next.capability_scope_root,[8;32]);
}
