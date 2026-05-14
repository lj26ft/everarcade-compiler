use execution_core::{
    civilization::{execute_civilization_genesis_flow, CivilizationGenesis},
    codec::{package_decode::decode_package, package_encode::encode_package},
};

fn genesis(seed: u8) -> CivilizationGenesis {
    CivilizationGenesis {
        civilization_id: [seed; 32],
        domain_root: [seed.wrapping_add(1); 32],
        constitution_root: [seed.wrapping_add(2); 32],
        treasury_root: [seed.wrapping_add(3); 32],
        fiscal_root: [seed.wrapping_add(4); 32],
        monetary_root: [seed.wrapping_add(5); 32],
        asset_root: [seed.wrapping_add(6); 32],
    }
}

#[test]
fn trailing_bytes_rejected() {
    let package = execute_civilization_genesis_flow(genesis(3));
    let mut bytes = encode_package(&package);
    bytes.extend_from_slice(&[1, 2, 3]);
    assert!(decode_package(&bytes).is_err());
}

#[test]
fn truncated_payload_rejected() {
    let package = execute_civilization_genesis_flow(genesis(3));
    let mut bytes = encode_package(&package);
    bytes.pop();
    assert!(decode_package(&bytes).is_err());
}
