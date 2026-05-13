use execution_core::abi::{validate_host_execution_input, HostExecutionInput, CURRENT_ABI_VERSION};

#[test]
fn host_abi_input_stable() {
    let input = HostExecutionInput { abi_version: CURRENT_ABI_VERSION, vm_input_root:[7;32], encoded_package: vec![1,2,3]};
    assert!(validate_host_execution_input(&input));
}
