use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostExecutionSession {
    pub session_id: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostExecutionContext {
    pub module_hash: String,
    pub state_root: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeExecutionBoundary {
    pub lifecycle: Vec<String>,
}

pub struct WasmRuntimeHost;
impl WasmRuntimeHost {
    pub fn lifecycle() -> RuntimeExecutionBoundary {
        RuntimeExecutionBoundary {
            lifecycle: vec![
                "LOAD_MODULE".to_string(),
                "VALIDATE_MODULE".to_string(),
                "INSTANTIATE".to_string(),
                "LOAD_STATE".to_string(),
                "EXECUTE".to_string(),
                "VALIDATE_MUTATIONS".to_string(),
                "APPLY_STATE".to_string(),
                "CHECKPOINT".to_string(),
                "PERSIST".to_string(),
                "EMIT_RECEIPT".to_string(),
                "ARCHIVE".to_string(),
            ],
        }
    }
}
