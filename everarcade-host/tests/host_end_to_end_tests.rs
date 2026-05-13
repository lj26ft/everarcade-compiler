use std::fs;

use everarcade_host::{package_loader::save_package, run_package_once, HostConfig};
use execution_core::civilization::{execute_civilization_genesis_flow, CivilizationGenesis};

fn fixture_package() -> execution_core::civilization::CivilizationPackage {
    execute_civilization_genesis_flow(CivilizationGenesis { civilization_id: [1;32], domain_root:[2;32], constitution_root:[3;32], treasury_root:[4;32], fiscal_root:[5;32], monetary_root:[6;32], asset_root:[7;32] })
}

#[test]
fn host_run_persists_and_queues_anchor() {
    let base = std::env::temp_dir().join("everarcade-host-e2e");
    let _ = fs::remove_dir_all(&base);
    fs::create_dir_all(&base).unwrap();
    let package_path = base.join("civilization_package.bin");
    save_package(&package_path, &fixture_package()).unwrap();

    let out = run_package_once(HostConfig::new(&package_path, base.join(".everarcade"))).unwrap();
    let rid = hex::encode(out.receipt.receipt_id);
    assert!(base.join(".everarcade/receipts").join(format!("{rid}.bin")).exists());
    assert!(base.join(".everarcade/anchors").join(format!("{rid}.json")).exists());
}
