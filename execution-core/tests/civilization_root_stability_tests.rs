use execution_core::civilization::{civilization_root::compute_civilization_root, CivilizationGenesis};
#[test]
fn civilization_root_stable() {
 let g = CivilizationGenesis { civilization_id:[1;32], domain_root:[2;32], constitution_root:[3;32], treasury_root:[4;32], fiscal_root:[5;32], monetary_root:[6;32], asset_root:[7;32]};
 assert_eq!(compute_civilization_root(&g), compute_civilization_root(&g));
}
