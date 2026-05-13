use execution_core::vm::{vm_input::VmExecutionInput, vm_root::compute_vm_input_root};

#[test]
fn vm_input_root_is_deterministic() {
    let input = VmExecutionInput { package_manifest_root: [1;32], civilization_root:[2;32], replay_root:[3;32], checkpoint_root:[4;32], payload_root:[5;32] };
    assert_eq!(compute_vm_input_root(&input), compute_vm_input_root(&input));
}
