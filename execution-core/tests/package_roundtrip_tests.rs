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
fn package_roundtrip_stable() {
    let package = execute_civilization_genesis_flow(genesis(1));
    let bytes = encode_package(&package);
    let decoded = decode_package(&bytes).unwrap();
    let reencoded = encode_package(&decoded);
    assert_eq!(bytes, reencoded);
}

#[test]
fn package_roundtrip_100_iterations_stable() {
    let package = execute_civilization_genesis_flow(genesis(1));
    let mut bytes = encode_package(&package);
    for _ in 0..100 {
        let decoded = decode_package(&bytes).unwrap();
        bytes = encode_package(&decoded);
    }
    assert_eq!(bytes, encode_package(&package));
}
