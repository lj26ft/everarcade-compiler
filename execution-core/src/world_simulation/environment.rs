use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EnvironmentState {
    pub climate_index: i64,
    pub replay_root: String,
}
