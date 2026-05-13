use execution_core::civilization::{execute_civilization_genesis_flow, CivilizationGenesis};
#[test]
fn sync_convergence_same_replay_root() {
 let g = CivilizationGenesis { civilization_id:[8;32], domain_root:[1;32], constitution_root:[2;32], treasury_root:[3;32], fiscal_root:[4;32], monetary_root:[5;32], asset_root:[6;32]};
 let a = execute_civilization_genesis_flow(g.clone());
 let b = execute_civilization_genesis_flow(g);
 assert_eq!(a.replay_root, b.replay_root);
}
