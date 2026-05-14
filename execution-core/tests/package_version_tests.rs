use execution_core::{
    civilization::{execute_civilization_genesis_flow, CivilizationGenesis},
    codec::{package_decode::decode_package, package_encode::encode_package},
};
fn genesis(seed: u8) -> CivilizationGenesis { CivilizationGenesis { civilization_id:[seed;32], domain_root:[seed+1;32], constitution_root:[seed+2;32], treasury_root:[seed+3;32], fiscal_root:[seed+4;32], monetary_root:[seed+5;32], asset_root:[seed+6;32] }}
#[test]
fn unsupported_version_rejected() { let package = execute_civilization_genesis_flow(genesis(2)); let mut bytes=encode_package(&package); bytes[0]=0x02; assert!(decode_package(&bytes).is_err()); }
