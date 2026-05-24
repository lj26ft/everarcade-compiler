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
    pub lifecycle: Vec<&'static str>,
}

pub struct WasmRuntimeHost;
impl WasmRuntimeHost {
    pub fn lifecycle() -> RuntimeExecutionBoundary {
        RuntimeExecutionBoundary {
            lifecycle: vec![
                "LOAD_MODULE",
                "VALIDATE_MODULE",
                "INSTANTIATE",
                "LOAD_STATE",
                "EXECUTE",
                "VALIDATE_MUTATIONS",
                "APPLY_STATE",
                "CHECKPOINT",
                "PERSIST",
                "EMIT_RECEIPT",
                "ARCHIVE",
            ],
        }
    }
}
