use crate::distributed_receipts::{
    execution_receipt::DistributedExecutionReceipt,
    receipt_codec::{decode_canonical, encode_canonical},
    receipt_manifest::DistributedReceiptManifest,
    receipt_store_error::ReceiptStoreError,
    Hash,
};
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub struct DistributedReceiptDiskStore {
    base: PathBuf,
}

impl DistributedReceiptDiskStore {
    pub fn new(base: impl AsRef<Path>) -> Result<Self, ReceiptStoreError> {
        let base = base.as_ref().join(".everarcade/distributed_receipts");
        fs::create_dir_all(base.join("receipts"))?;
        fs::create_dir_all(base.join("index"))?;
        if !base.join("manifest.json").exists() {
            fs::write(
                base.join("manifest.json"),
                serde_json::to_vec_pretty(&DistributedReceiptManifest::default())?,
            )?;
        }
        Ok(Self { base })
    }

    pub fn persist_receipt(
        &self,
        replay_root: Hash,
        checkpoint_root: Hash,
        receipt: &DistributedExecutionReceipt,
    ) -> Result<(), ReceiptStoreError> {
        let bytes = encode_canonical(receipt)?;
        let receipt_file = self
            .base
            .join("receipts")
            .join(format!("{}.bin", hex::encode(receipt.receipt_root)));
        let tmp = receipt_file.with_extension("tmp");
        fs::write(&tmp, &bytes)?;
        fs::rename(tmp, &receipt_file)?;

        let continuity_file = self
            .base
            .join("index")
            .join(format!("{}.json", hex::encode(receipt.receipt_root)));
        fs::write(
            continuity_file,
            serde_json::to_vec_pretty(&(replay_root, checkpoint_root))?,
        )?;

        let mut manifest = self.load_manifest()?;
        manifest.receipt_count += 1;
        manifest.latest_receipt_root = Some(receipt.receipt_root);
        manifest.latest_replay_root = Some(replay_root);
        manifest.latest_checkpoint_root = Some(checkpoint_root);
        fs::write(
            self.base.join("manifest.json"),
            serde_json::to_vec_pretty(&manifest)?,
        )?;

        let round_trip = decode_canonical(&fs::read(receipt_file)?)?;
        if &round_trip != receipt {
            return Err(ReceiptStoreError::Validation("receipt round trip mismatch"));
        }
        Ok(())
    }

    pub fn load_manifest(&self) -> Result<DistributedReceiptManifest, ReceiptStoreError> {
        Ok(serde_json::from_slice(&fs::read(
            self.base.join("manifest.json"),
        )?)?)
    }
}
