use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::error::HostError;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeManifest {
    pub node_name: String,
    pub protocol_version: u64,
    pub state_root: String,
    pub last_receipt_root: Option<String>,
    pub last_checkpoint_root: Option<String>,
    pub last_anchor_root: Option<String>,
}

impl NodeManifest {
    pub fn new(node_name: &str) -> Self {
        Self {
            node_name: node_name.to_string(),
            protocol_version: 1,
            state_root: "0000000000000000000000000000000000000000000000000000000000000000".into(),
            last_receipt_root: None,
            last_checkpoint_root: None,
            last_anchor_root: None,
        }
    }
}

pub fn write_node_manifest(state_root: &Path, manifest: &NodeManifest) -> Result<(), HostError> {
    fs::write(
        state_root.join("node_manifest.json"),
        serde_json::to_vec_pretty(manifest).map_err(|_| HostError::InvalidStateFolder)?,
    )?;
    Ok(())
}

pub fn read_node_manifest(state_root: &Path) -> Result<NodeManifest, HostError> {
    let bytes = fs::read(state_root.join("node_manifest.json"))?;
    serde_json::from_slice(&bytes).map_err(|_| HostError::InvalidStateFolder)
}
