use serde::{Deserialize, Serialize};

use super::{profile::OperatorProfile, validation::validate_config};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorConfig {
    pub node_name: String,
    pub state_path: String,
    pub profile: OperatorProfile,
    pub dry_run: bool,
    pub xrpl_enabled: bool,
    pub ipfs_enabled: bool,
    pub evernode_enabled: bool,
}

impl Default for OperatorConfig {
    fn default() -> Self {
        let profile = OperatorProfile::Local;
        Self {
            node_name: "everarcade-node".into(),
            state_path: profile.state_layout().into(),
            profile,
            dry_run: true,
            xrpl_enabled: false,
            ipfs_enabled: false,
            evernode_enabled: false,
        }
    }
}

impl OperatorConfig {
    pub fn live_testnet(node_name: impl Into<String>) -> Self {
        Self {
            node_name: node_name.into(),
            state_path: OperatorProfile::Live.state_layout().into(),
            profile: OperatorProfile::Live,
            dry_run: false,
            xrpl_enabled: true,
            ipfs_enabled: true,
            evernode_enabled: true,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        validate_config(self)
    }
}
