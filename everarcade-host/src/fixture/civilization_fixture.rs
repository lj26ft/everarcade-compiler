use execution_core::civilization::{
    execute_civilization_genesis_flow, CivilizationGenesis, CivilizationPackage,
};

pub fn fixture_genesis() -> CivilizationGenesis {
    CivilizationGenesis {
        civilization_id: [1; 32],
        domain_root: [2; 32],
        constitution_root: [3; 32],
        treasury_root: [4; 32],
        fiscal_root: [5; 32],
        monetary_root: [6; 32],
        asset_root: [7; 32],
    }
}

pub fn generate_civilization_fixture_package() -> CivilizationPackage {
    execute_civilization_genesis_flow(fixture_genesis())
}
