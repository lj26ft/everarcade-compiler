use execution_core::vm::vm_input::VmExecutionInput;
use execution_core::wasm::wasm_boundary::execute_wasm_boundary;

#[test]
fn wasm_boundary_encodes_receipt() {
    let input = VmExecutionInput {
        package_manifest_root: [1; 32],
        civilization_root: [2; 32],
        pre_state_root: [3; 32],
        prior_replay_root_value: [3; 32],
        checkpoint_root: [4; 32],
        payload_root: [5; 32],
    };
    let (_receipt, bytes) = execute_wasm_boundary(&input);
    assert!(!bytes.is_empty());
}
