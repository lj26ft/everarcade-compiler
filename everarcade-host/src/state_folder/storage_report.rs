use std::{fs, path::Path};

use crate::error::HostError;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StorageReport {
    pub receipt_count: u64,
    pub checkpoint_count: u64,
    pub anchor_count: u64,
    pub total_bytes: u64,
}

fn count_files(path: &Path) -> Result<u64, HostError> {
    if !path.exists() {
        return Ok(0);
    }
    Ok(fs::read_dir(path)?.filter_map(Result::ok).count() as u64)
}

fn dir_size(path: &Path) -> Result<u64, HostError> {
    if !path.exists() {
        return Ok(0);
    }
    let mut total = 0_u64;
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let md = entry.metadata()?;
        if md.is_file() {
            total = total.saturating_add(md.len());
        }
    }
    Ok(total)
}

pub fn storage_report(state: &Path) -> Result<StorageReport, HostError> {
    let receipts = state.join("receipts");
    let checkpoints = state.join("checkpoints");
    let anchors = state.join("anchors");

    Ok(StorageReport {
        receipt_count: count_files(&receipts)?,
        checkpoint_count: count_files(&checkpoints)?,
        anchor_count: count_files(&anchors)?,
        total_bytes: dir_size(&receipts)? + dir_size(&checkpoints)? + dir_size(&anchors)?,
    })
}
