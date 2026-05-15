use crate::distributed_receipts::{
    receipt_manifest::DistributedReceiptManifest, receipt_store_error::ReceiptStoreError, Hash,
};
use std::{fs, path::Path};

pub fn rebuild_manifest_from_receipts(
    base: impl AsRef<Path>,
) -> Result<DistributedReceiptManifest, ReceiptStoreError> {
    let base = base.as_ref().join(".everarcade/distributed_receipts");
    let dir = base.join("receipts");
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

    let latest_receipt_root = roots.last().copied();
    let (latest_replay_root, latest_checkpoint_root) =
        if let Some(receipt_root) = latest_receipt_root {
            let continuity = base
                .join("index")
                .join(format!("{}.json", hex::encode(receipt_root)));
            if continuity.exists() {
                let (replay_root, checkpoint_root): (Hash, Hash) =
                    serde_json::from_slice(&fs::read(continuity)?)?;
                (Some(replay_root), Some(checkpoint_root))
            } else {
                (Some(receipt_root), Some(receipt_root))
            }
        } else {
            (None, None)
        };

    Ok(DistributedReceiptManifest {
        receipt_count: roots.len() as u64,
        latest_receipt_root,
        latest_replay_root,
        latest_checkpoint_root,
    })
}
