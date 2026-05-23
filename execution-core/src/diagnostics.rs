use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExecutionProfile {
    pub profile_version: u32,
    pub operation_name: String,
    pub operation_index: u64,
    pub input_record_count: u64,
    pub output_record_count: u64,
    pub dag_execution_count: u64,
    pub replay_operations: u64,
    pub state_diff_count: u64,
    pub receipt_count: u64,
    pub wasm_call_count: u64,
    pub fuel_consumed: u64,
    pub memory_pages_touched: u64,
    pub estimated_memory_bytes: u64,
    pub diagnostic_duration_ns: u128,
    pub warnings: Vec<String>,
}

impl Default for ExecutionProfile {
    fn default() -> Self {
        Self {
            profile_version: 1,
            operation_name: String::new(),
            operation_index: 0,
            input_record_count: 0,
            output_record_count: 0,
            dag_execution_count: 0,
            replay_operations: 0,
            state_diff_count: 0,
            receipt_count: 0,
            wasm_call_count: 0,
            fuel_consumed: 0,
            memory_pages_touched: 0,
            estimated_memory_bytes: 0,
            diagnostic_duration_ns: 0,
            warnings: Vec::new(),
        }
    }
}

pub type DagExecutionProfile = ExecutionProfile;
pub type WasmBoundaryProfile = ExecutionProfile;
pub type ReplayProfile = ExecutionProfile;
pub type ReceiptProfile = ExecutionProfile;
pub type StateDiffProfile = ExecutionProfile;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OperatorDiagnosticEnvelope<T: Serialize> {
    pub component: String,
    pub event: String,
    pub sequence: u64,
    pub deterministic: bool,
    pub profile: T,
}

impl<T: Serialize> OperatorDiagnosticEnvelope<T> {
    pub fn json_line(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}
