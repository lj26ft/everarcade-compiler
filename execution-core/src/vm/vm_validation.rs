use super::vm_input::VmExecutionInput;

pub fn validate_vm_execution_input(input: &VmExecutionInput) -> bool {
    input.package_manifest_root != [0; 32] && input.civilization_root != [0; 32]
}
