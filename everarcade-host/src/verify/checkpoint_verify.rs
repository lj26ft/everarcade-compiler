use std::path::Path;

use crate::{error::HostError, state_folder::node_manifest::NodeManifest};

pub fn verify_checkpoint(state: &Path, manifest: &NodeManifest) -> Result<bool, HostError> {
    let cp = manifest
        .last_checkpoint_root
        .as_ref()
        .ok_or(HostError::InvalidCheckpoint)?;
    Ok(state.join("checkpoints").join(format!("{cp}.bin")).exists())
}
