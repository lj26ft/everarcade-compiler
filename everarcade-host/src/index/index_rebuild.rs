use std::{fs, path::Path};

use crate::error::HostError;

#[derive(Clone, Debug, Default)]
pub struct IndexRebuildReport {
    pub rebuilt_receipts: u64,
    pub rebuilt_checkpoints: u64,
    pub rebuilt_anchors: u64,
}

pub fn rebuild_indexes(state_root: &Path) -> Result<IndexRebuildReport, HostError> {
    let receipts = fs::read_dir(state_root.join("receipts"))?.count() as u64;
    let checkpoints = fs::read_dir(state_root.join("checkpoints"))?.count() as u64;
    let anchors = fs::read_dir(state_root.join("anchors"))?.count() as u64;
    fs::write(
        state_root.join("manifests/receipt.index"),
        receipts.to_string(),
    )?;
    fs::write(
        state_root.join("manifests/checkpoint.index"),
        checkpoints.to_string(),
    )?;
    fs::write(
        state_root.join("manifests/anchor.index"),
        anchors.to_string(),
    )?;
    Ok(IndexRebuildReport {
        rebuilt_receipts: receipts,
        rebuilt_checkpoints: checkpoints,
        rebuilt_anchors: anchors,
    })
}
