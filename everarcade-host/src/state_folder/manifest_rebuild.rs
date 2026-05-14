use std::path::Path;

use crate::error::HostError;

use super::{
    manifest_scan::latest_root_from_dir,
    node_manifest::{read_node_manifest, write_node_manifest, NodeManifest},
};

pub type Hash = [u8; 32];

#[derive(Clone, Debug)]
pub struct ManifestRepairResult {
    pub repaired: bool,
    pub latest_receipt_root: Option<Hash>,
    pub latest_checkpoint_root: Option<Hash>,
}

fn decode_hash(maybe_hex: Option<String>) -> Option<Hash> {
    let s = maybe_hex?;
    let bytes = hex::decode(s).ok()?;
    if bytes.len() != 32 {
        return None;
    }
    let mut out = [0u8; 32];
    out.copy_from_slice(&bytes);
    Some(out)
}

pub fn repair_manifest(state_root: &Path) -> Result<ManifestRepairResult, HostError> {
    let mut manifest =
        read_node_manifest(state_root).unwrap_or_else(|_| NodeManifest::new("everarcade-node"));
    let latest_receipt = latest_root_from_dir(state_root, "receipts");
    let latest_checkpoint = latest_root_from_dir(state_root, "checkpoints");
    let latest_anchor = latest_root_from_dir(state_root, "anchors");

    let repaired = manifest.last_receipt_root != latest_receipt
        || manifest.last_checkpoint_root != latest_checkpoint
        || manifest.last_anchor_root != latest_anchor;

    manifest.last_receipt_root = latest_receipt.clone();
    manifest.last_checkpoint_root = latest_checkpoint.clone();
    manifest.last_anchor_root = latest_anchor;
    write_node_manifest(state_root, &manifest)?;

    Ok(ManifestRepairResult {
        repaired,
        latest_receipt_root: decode_hash(latest_receipt),
        latest_checkpoint_root: decode_hash(latest_checkpoint),
    })
}
