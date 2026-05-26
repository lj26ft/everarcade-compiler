use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HookBinding {
    pub hook_name: String,
    pub hook_hash: String,
    pub enabled: bool,
}
