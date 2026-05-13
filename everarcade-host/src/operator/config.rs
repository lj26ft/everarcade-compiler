use serde::{Deserialize, Serialize};

use super::validation::validate_config;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorConfig {
    pub node_name: String,
    pub state_path: String,
    pub dry_run: bool,
    pub xrpl_enabled: bool,
    pub ipfs_enabled: bool,
    pub evernode_enabled: bool,
}

impl Default for OperatorConfig {
    fn default() -> Self {
        Self {
            node_name: "everarcade-node".into(),
            state_path: ".everarcade".into(),
            dry_run: true,
            xrpl_enabled: false,
            ipfs_enabled: false,
            evernode_enabled: false,
        }
    }
}

impl OperatorConfig {
    pub fn validate(&self) -> Result<(), String> {
        validate_config(self)
    }
}
