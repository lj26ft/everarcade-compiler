pub mod vm_execution;
pub mod vm_input;
pub mod vm_output;
pub mod vm_receipt;
pub mod vm_root;
pub mod vm_validation;

pub use vm_execution::execute_vm_boundary;
pub use vm_input::VmExecutionInput;
pub use vm_output::VmExecutionOutput;
pub use vm_receipt::{compute_vm_receipt_root, validate_vm_receipt, VmExecutionReceipt};
