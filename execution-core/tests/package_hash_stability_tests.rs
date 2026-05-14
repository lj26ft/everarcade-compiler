use execution_core::{
    civilization::{execute_civilization_genesis_flow, CivilizationGenesis},
    codec::package_validation::package_root,
};
fn genesis(seed: u8) -> CivilizationGenesis { CivilizationGenesis { civilization_id:[seed;32], domain_root:[seed+1;32], constitution_root:[seed+2;32], treasury_root:[seed+3;32], fiscal_root:[seed+4;32], monetary_root:[seed+5;32], asset_root:[seed+6;32] }}
#[test]
fn package_hash_stable_for_same_generation() { let a=execute_civilization_genesis_flow(genesis(4)); let b=execute_civilization_genesis_flow(genesis(4)); assert_eq!(package_root(&a), package_root(&b)); }
