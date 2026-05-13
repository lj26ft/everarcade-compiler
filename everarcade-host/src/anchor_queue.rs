use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::error::HostError;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnchorQueueItem {
    pub receipt_id_hex: String,
    pub xrpl_intent: serde_json::Value,
    pub evernode_intent: serde_json::Value,
}

pub fn queue_anchor_intent(dir: &Path, item: &AnchorQueueItem) -> Result<std::path::PathBuf, HostError> {
    let path = dir.join(format!("{}.json", item.receipt_id_hex));
    fs::write(&path, serde_json::to_vec_pretty(item).expect("serialize queue item"))?;
    Ok(path)
}
