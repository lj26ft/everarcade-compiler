use execution_core::abi::{HostExecutionOutput, CURRENT_ABI_VERSION};
use execution_core::external::anchor_emission::emit_external_anchor_receipt;
use execution_core::vm::vm_execution::execute_vm_boundary;
use execution_core::vm::vm_input::VmExecutionInput;

#[test]
fn vm_end_to_end_boundary_flow() {
    let input = VmExecutionInput {
        package_manifest_root: [1; 32],
        civilization_root: [2; 32],
        pre_state_root: [3; 32],
        prior_replay_root_value: [3; 32],
        checkpoint_root: [4; 32],
        payload_root: [5; 32],
    };
    let (receipt, output) = execute_vm_boundary(&input);
    assert_eq!(receipt.anchor_root, output.external_anchor_root);
    let anchor = emit_external_anchor_receipt(output.external_anchor_root);
    assert_eq!(anchor.anchor_root, output.external_anchor_root);

    let host_output = HostExecutionOutput {
        abi_version: CURRENT_ABI_VERSION,
        vm_output_root: output.vm_receipt_root,
        encoded_receipt: bincode::serialize(&receipt).unwrap(),
    };
    assert!(!host_output.encoded_receipt.is_empty());
}
