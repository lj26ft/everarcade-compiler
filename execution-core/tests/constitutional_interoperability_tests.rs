use execution_core::{simulation::treaty_simulation::treaty_continuity_valid, treaty::treaty::ExecutionTreaty};
#[test]
fn constitutional_interoperability_has_replayable_treaties() {
 let _t=ExecutionTreaty{treaty_id:[1;32],participating_domains:vec![[2;32],[3;32]],constitutional_scope_root:[4;32],capability_scope_root:[5;32],arbitration_root:[6;32]};
 assert!(treaty_continuity_valid(&[[10;32],[11;32]]));
}
