use std::collections::BTreeMap;

use execution_core::hashing;
use execution_core::package::bundle::ExecutionPackage;
use execution_core::package::export::export_package;
use execution_core::package::import::import_package;
use execution_core::package::manifest::ExecutionManifest;
use execution_core::package::verify::verify_replay;
use execution_core::state_engine::proof::create_inclusion_proof;
use execution_core::state_engine::snapshot::StateSnapshot;
use execution_core::verifier::node::ContractWasm;
use execution_core::{execute, ExecutionNode, ExecutionPlan, VmInput, ABI_VERSION};

fn sample_package(epoch: u64) -> ExecutionPackage {
    let mut state = BTreeMap::new();
    state.insert("score".to_string(), "0".to_string());
    let plan = ExecutionPlan {
        nodes: vec![ExecutionNode {
            id: "n1".to_string(),
            action: "set".to_string(),
            payload: serde_json::json!({"key":"score", "value":"42"}),
            deps: vec![],
        }],
    };

    let output = execute::execute_vm(VmInput {
        protocol_epoch_id: epoch,
        state: state.clone(),
        plan: plan.clone(),
    });
    let snapshot = StateSnapshot::new(output.updated_state.clone(), None);
    let contracts = vec![ContractWasm {
        contract_id: "c1".to_string(),
        wasm_bytes: vec![1, 2, 3],
    }];
    let contract_hashes = contracts
        .iter()
        .map(|c| hashing::compute_contract_hash(&c.wasm_bytes))
        .collect::<Vec<_>>();

    let mut pkg = ExecutionPackage {
        manifest: ExecutionManifest {
            package_id: "pkg-1".to_string(),
            protocol_epoch: epoch,
            abi_version: ABI_VERSION.to_string(),
            snapshot_version: "state-snapshot-v1".to_string(),
            execution_root: output.receipt.execution_root.clone(),
            state_root: snapshot.state_root.clone(),
            contract_hashes,
            package_hash: String::new(),
        },
        dag: plan,
        contracts,
        snapshot,
        receipts: vec![output.receipt],
        proofs: vec![create_inclusion_proof("score", "42")],
    };
    pkg.manifest.package_hash = pkg.recompute_package_hash().unwrap();
    pkg
}

#[test]
fn test_package_export_import() {
    let package = sample_package(1);
    let bytes = export_package(&package).unwrap();
    let imported = import_package(&bytes, 1).unwrap();
    assert_eq!(
        imported.manifest.package_hash,
        package.manifest.package_hash
    );
}

#[test]
fn test_package_replay() {
    let package = sample_package(2);
    assert!(verify_replay(&package).is_ok());
}

#[test]
fn test_package_hash_stability() {
    let package_a = sample_package(3);
    let package_b = sample_package(3);
    assert_eq!(
        package_a.manifest.package_hash,
        package_b.manifest.package_hash
    );
}

#[test]
fn test_cross_machine_package_replay() {
    let package = sample_package(4);
    let bytes = export_package(&package).unwrap();
    let imported = import_package(&bytes, 4).unwrap();
    assert!(verify_replay(&imported).is_ok());
}

#[test]
fn test_epoch_package_compatibility() {
    let package = sample_package(5);
    let bytes = export_package(&package).unwrap();
    assert!(import_package(&bytes, 6).is_err());
}

#[test]
fn test_invalid_package_rejection() {
    let package = sample_package(7);
    let mut bytes = export_package(&package).unwrap();
    bytes[10] ^= 0xFF;
    assert!(import_package(&bytes, 7).is_err());
}
