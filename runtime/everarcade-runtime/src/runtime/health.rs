use crate::runtime::lifecycle::RuntimeState;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RuntimeHealth {
    pub runtime_version: String,
    pub package_hash: String,
    pub package_version: String,
    pub world_id: String,
    pub world_root: String,
    pub replay_cursor: u64,
    pub journal_height: u64,
    pub checkpoint_height: u64,
    pub checkpoint_age_ms: u128,
    pub latest_receipt: Option<String>,
    pub runtime_state: RuntimeState,
    pub memory_usage_bytes: u64,
    pub disk_usage_bytes: u64,
    pub recovery_status: String,
}

impl RuntimeHealth {
    pub fn new(runtime_version: String, world_id: String) -> Self {
        Self {
            runtime_version,
            package_hash: String::new(),
            package_version: String::new(),
            world_id,
            world_root: String::new(),
            replay_cursor: 0,
            journal_height: 0,
            checkpoint_height: 0,
            checkpoint_age_ms: 0,
            latest_receipt: None,
            runtime_state: RuntimeState::Booting,
            memory_usage_bytes: 0,
            disk_usage_bytes: 0,
            recovery_status: "not-started".into(),
        }
    }
}
