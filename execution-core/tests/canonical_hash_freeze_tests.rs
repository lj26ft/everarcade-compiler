use execution_core::civilization::{
    civilization_root::compute_civilization_root, CivilizationGenesis,
};
#[test]
fn canonical_hash_freeze_stable_for_same_input() {
    let g = CivilizationGenesis {
        civilization_id: [10; 32],
        domain_root: [11; 32],
        constitution_root: [12; 32],
        treasury_root: [13; 32],
        fiscal_root: [14; 32],
        monetary_root: [15; 32],
        asset_root: [16; 32],
    };
    assert_eq!(compute_civilization_root(&g), compute_civilization_root(&g));
}
