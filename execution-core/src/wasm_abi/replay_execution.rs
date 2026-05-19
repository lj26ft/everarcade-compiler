use crate::runtime_commit::{replay_verify, CommitOutput, ReplayReport};

use super::errors::WasmAbiError;

pub fn verify_execution_replay(entries: &[CommitOutput]) -> Result<ReplayReport, WasmAbiError> {
    replay_verify(entries).map_err(|e| WasmAbiError::Runtime(e.to_string()))
}
