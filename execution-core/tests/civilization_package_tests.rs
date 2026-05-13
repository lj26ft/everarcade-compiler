use execution_core::civilization::{civilization_validation::validate_civilization_package, execute_civilization_genesis_flow, CivilizationGenesis};
fn h(b: u8) -> [u8; 32] { [b; 32] }
#[test]
fn package_validates() {
let g = CivilizationGenesis { civilization_id: h(11), domain_root: h(12), constitution_root: h(13), treasury_root: h(14), fiscal_root: h(15), monetary_root: h(16), asset_root: h(17) };
let p = execute_civilization_genesis_flow(g);
assert!(validate_civilization_package(&p));
}
