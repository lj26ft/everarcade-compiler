use crate::vm::vm_receipt::VmExecutionReceipt;

pub fn decode_wasm_receipt(bytes: &[u8]) -> Option<VmExecutionReceipt> {
    bincode::deserialize(bytes).ok()
}
