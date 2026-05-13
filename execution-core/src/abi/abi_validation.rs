use super::abi_version::CURRENT_ABI_VERSION;
use super::{HostExecutionInput, HostExecutionOutput};

pub fn validate_host_execution_input(input: &HostExecutionInput) -> bool {
    input.abi_version == CURRENT_ABI_VERSION && !input.encoded_package.is_empty()
}

pub fn validate_host_execution_output(output: &HostExecutionOutput) -> bool {
    output.abi_version == CURRENT_ABI_VERSION && !output.encoded_receipt.is_empty()
}
