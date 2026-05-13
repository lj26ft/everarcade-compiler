use crate::abi::{HostExecutionInput, HostExecutionOutput};

pub struct HostBoundaryRecord {
    pub input: HostExecutionInput,
    pub output: HostExecutionOutput,
}
