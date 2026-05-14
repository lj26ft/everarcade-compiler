use std::path::Path;

use crate::{error::HostError, state_folder::node_manifest::NodeManifest};

pub fn verify_anchor(state: &Path, manifest: &NodeManifest) -> Result<bool, HostError> {
    let anchor = manifest
        .last_anchor_root
        .as_ref()
        .ok_or(HostError::AnchorIntentMissing)?;
    Ok(state
        .join("anchors")
        .join(format!("{anchor}.json"))
        .exists())
}
