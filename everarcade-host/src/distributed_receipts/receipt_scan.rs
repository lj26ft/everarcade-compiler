use crate::distributed_receipts::{
    receipt_manifest::DistributedReceiptManifest, receipt_store_error::ReceiptStoreError, Hash,
};
use std::{fs, path::Path};

pub fn rebuild_manifest_from_receipts(
    base: impl AsRef<Path>,
) -> Result<DistributedReceiptManifest, ReceiptStoreError> {
    let dir = base
        .as_ref()
        .join(".everarcade/distributed_receipts/receipts");
    let mut roots: Vec<Hash> = fs::read_dir(dir)?
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            entry
                .path()
                .file_stem()
                .map(|s| s.to_string_lossy().to_string())
        })
        .filter_map(|hex_root| {
            let mut out = [0u8; 32];
            let decoded = hex::decode(hex_root).ok()?;
            if decoded.len() != 32 {
                return None;
            }
            out.copy_from_slice(&decoded);
            Some(out)
        })
        .collect();
    roots.sort();
    Ok(DistributedReceiptManifest {
        receipt_count: roots.len() as u64,
        latest_receipt_root: roots.last().copied(),
        latest_replay_root: roots.last().copied(),
        latest_checkpoint_root: roots.last().copied(),
    })
}
