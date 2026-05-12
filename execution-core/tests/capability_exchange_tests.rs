use execution_core::capability::{capability::Capability, capability_exchange::exchange_capability, sovereign_scope::is_treaty_scoped};
#[test]
fn capability_exchange_remains_treaty_scoped() {
 let cap=Capability{capability_id:[1;32],issuing_domain:[2;32],authority_scope:[3;32],parent_capability:None,revocation_root:None};
 let exchanged=exchange_capability(&cap,[7;32],[8;32]);
 assert!(is_treaty_scoped(&exchanged,[8;32]));
}
