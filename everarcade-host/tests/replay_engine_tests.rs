use execution_core::vm::{execute_vm_boundary, VmExecutionInput};

use everarcade_host::{fixture::civilization_fixture::generate_civilization_fixture_package, replay_engine::verify_receipt_replay_from_artifacts};

#[test]
fn replay_engine_reconstructs_identical_receipt() {
    let package = generate_civilization_fixture_package();
    let input = VmExecutionInput {
        package_manifest_root: package.execution_root,
        civilization_root: package.execution_root,
        replay_root: package.replay_root,
        checkpoint_root: package.checkpoint_root,
        payload_root: package.proof_root,
    };

    let (receipt, _) = execute_vm_boundary(&input);
    let report = verify_receipt_replay_from_artifacts(&package, &receipt).expect("replay report");

    assert!(report.receipt_canonical_valid);
    assert!(report.package_matches_receipt);
    assert!(report.deterministic_replay_match);
    assert!(report.verified());
}

#[test]
fn replay_engine_detects_tampered_receipt() {
    let package = generate_civilization_fixture_package();
    let input = VmExecutionInput {
        package_manifest_root: package.execution_root,
        civilization_root: package.execution_root,
        replay_root: package.replay_root,
        checkpoint_root: package.checkpoint_root,
        payload_root: package.proof_root,
    };

    let (mut receipt, _) = execute_vm_boundary(&input);
    receipt.next_replay_root[0] ^= 0xFF;

    let report = verify_receipt_replay_from_artifacts(&package, &receipt).expect("replay report");

    assert!(!report.receipt_canonical_valid);
    assert!(!report.deterministic_replay_match);
    assert!(!report.verified());
}
