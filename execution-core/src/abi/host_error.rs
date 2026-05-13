use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HostExecutionError {
    AbiVersionMismatch,
    InvalidVmInputRoot,
    DecodeFailure,
    EncodeFailure,
}
