use execution_core::civilization::{execute_civilization_genesis_flow, CivilizationGenesis};

fn h(b: u8) -> [u8; 32] {
    [b; 32]
}

#[test]
fn same_genesis_same_package_roots() {
    let g = CivilizationGenesis {
        civilization_id: h(1),
        domain_root: h(2),
        constitution_root: h(3),
        treasury_root: h(4),
        fiscal_root: h(5),
        monetary_root: h(6),
        asset_root: h(7),
    };
    let a = execute_civilization_genesis_flow(g.clone());
    let b = execute_civilization_genesis_flow(g);
    assert_eq!(a.execution_root, b.execution_root);
    assert_eq!(a.replay_root, b.replay_root);
}
