use crate::vm::vm_execution::execute_vm_boundary;
use crate::vm::vm_input::VmExecutionInput;
use crate::vm::vm_receipt::VmExecutionReceipt;

pub fn execute_wasm_boundary(input: &VmExecutionInput) -> (VmExecutionReceipt, Vec<u8>) {
    let (receipt, _) = execute_vm_boundary(input);
    let encoded = bincode::serialize(&receipt).unwrap_or_default();
    (receipt, encoded)
}
