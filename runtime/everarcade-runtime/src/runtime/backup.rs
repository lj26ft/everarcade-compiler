use crate::runtime::{CheckpointManager, PersistenceManager};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BackupManifest {
    pub backup_id: String,
    pub checkpoint_sequence: u64,
    pub checkpoint_hash: String,
    pub created_at_ms: u128,
}

#[derive(Clone, Debug)]
pub struct BackupManager {
    pub dir: PathBuf,
    pub checkpoints: CheckpointManager,
    pub persistence: PersistenceManager,
}

impl BackupManager {
    pub fn backup(&self) -> Result<BackupManifest> {
        let cp = self
            .checkpoints
            .latest()?
            .ok_or_else(|| anyhow!("no checkpoint available for backup"))?;
        let created_at_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        let backup_id = hex::encode(Sha256::digest(
            format!("{}:{}", cp.manifest.checkpoint_hash, created_at_ms).as_bytes(),
        ));
        let manifest = BackupManifest {
            backup_id: backup_id.clone(),
            checkpoint_sequence: cp.manifest.sequence,
            checkpoint_hash: cp.manifest.checkpoint_hash.clone(),
            created_at_ms,
        };
        let backup_dir = self.dir.join(&backup_id);
        self.persistence
            .write_versioned(backup_dir.join("checkpoint.json"), &cp)?;
        self.persistence
            .write_versioned(backup_dir.join("manifest.json"), &manifest)?;
        Ok(manifest)
    }

    pub fn verify(&self, backup_id: &str) -> Result<BackupManifest> {
        let manifest: BackupManifest = self
            .persistence
            .read_versioned(self.dir.join(backup_id).join("manifest.json"))?;
        let cp: crate::runtime::Checkpoint = self
            .persistence
            .read_versioned(self.dir.join(backup_id).join("checkpoint.json"))?;
        if manifest.checkpoint_hash != cp.manifest.checkpoint_hash {
            return Err(anyhow!("backup checkpoint hash mismatch"));
        }
        Ok(manifest)
    }
}
