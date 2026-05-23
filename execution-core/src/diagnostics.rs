use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct ExecutionProfile {
    pub operation_name: String,
    pub operation_index: u64,
    pub input_record_count: u64,
    pub output_record_count: u64,
    pub state_diff_count: u64,
    pub receipt_count: u64,
    pub wasm_call_count: u64,
    pub fuel_consumed: u64,
    pub memory_pages_touched: u64,
    pub diagnostic_duration_ns: u128,
    pub warnings: Vec<String>,
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
