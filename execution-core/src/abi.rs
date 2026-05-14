// FILE: execution-core/src/abi.rs
//
// EVERARCADE EXECUTION ABI v2 (FROZEN)
//

pub const ABI_VERSION: &str = "everarcade-execution-abi-v2";

pub const INPUT_SCHEMA_VERSION: &str = "v2";
pub const OUTPUT_SCHEMA_VERSION: &str = "v2";
pub const EXECUTION_SEMANTICS_VERSION: &str = "v2";
pub const STATE_MODEL_VERSION: &str = "v2";
pub const RECEIPT_FORMAT_VERSION: &str = "v2";

pub mod abi_validation;
pub mod abi_version;
pub mod host_error;
pub mod host_input;
pub mod host_output;
pub mod host_receipt;

pub use abi_validation::{validate_host_execution_input, validate_host_execution_output};
pub use abi_version::CURRENT_ABI_VERSION;
pub use host_error::HostExecutionError;
pub use host_input::HostExecutionInput;
pub use host_output::HostExecutionOutput;
pub use host_receipt::HostExecutionReceipt;
