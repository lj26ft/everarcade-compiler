use execution_core::civilization::{execute_civilization_genesis_flow, CivilizationGenesis};
#[test]
fn replay_root_is_stable() {
 let g = CivilizationGenesis { civilization_id:[9;32], domain_root:[1;32], constitution_root:[2;32], treasury_root:[3;32], fiscal_root:[4;32], monetary_root:[5;32], asset_root:[6;32]};
 let p1 = execute_civilization_genesis_flow(g.clone());
 let p2 = execute_civilization_genesis_flow(g);
 assert_eq!(p1.replay_root, p2.replay_root);
}
