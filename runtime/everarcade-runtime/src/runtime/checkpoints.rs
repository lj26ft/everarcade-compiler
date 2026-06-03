use crate::runtime::persistence::PersistenceManager;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct CheckpointManifest {
    pub sequence: u64,
    pub created_at_ms: u128,
    pub world_id: String,
    pub runtime_version: String,
    pub journal_position: u64,
    pub state_root: String,
    pub checkpoint_hash: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Checkpoint {
    pub manifest: CheckpointManifest,
    pub state_snapshot: Vec<u8>,
    pub world_metadata: serde_json::Value,
    pub runtime_metadata: serde_json::Value,
}

#[derive(Clone, Debug)]
pub struct CheckpointManager {
    dir: PathBuf,
    persistence: PersistenceManager,
    interval_ticks: u64,
}

impl CheckpointManager {
    pub fn new(dir: PathBuf, persistence: PersistenceManager, interval_ticks: u64) -> Self {
        Self {
            dir,
            persistence,
            interval_ticks,
        }
    }
    pub fn should_checkpoint(&self, tick: u64) -> bool {
        self.interval_ticks > 0 && tick % self.interval_ticks == 0
    }

    pub fn create(
        &self,
        sequence: u64,
        world_id: &str,
        runtime_version: &str,
        journal_position: u64,
        state_root: String,
        state_snapshot: Vec<u8>,
        world_metadata: serde_json::Value,
    ) -> Result<Checkpoint> {
        let created_at_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        let checkpoint_hash = Self::hash(sequence, journal_position, &state_root, &state_snapshot);
        let manifest = CheckpointManifest {
            sequence,
            created_at_ms,
            world_id: world_id.to_string(),
            runtime_version: runtime_version.to_string(),
            journal_position,
            state_root,
            checkpoint_hash,
        };
        let checkpoint = Checkpoint {
            manifest,
            state_snapshot,
            world_metadata,
            runtime_metadata: serde_json::json!({"layout_version": 1}),
        };
        self.persistence
            .write_versioned(self.path_for(sequence), &checkpoint)?;
        Ok(checkpoint)
    }

    pub fn latest(&self) -> Result<Option<Checkpoint>> {
        if !self.dir.exists() {
            return Ok(None);
        }
        let mut checkpoints = std::fs::read_dir(&self.dir)?
            .filter_map(Result::ok)
            .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("json"))
            .collect::<Vec<_>>();
        checkpoints.sort_by_key(|e| e.path());
        match checkpoints.last() {
            Some(e) => Ok(Some(self.load_path(e.path())?)),
            None => Ok(None),
        }
    }

    pub fn load_path(&self, path: PathBuf) -> Result<Checkpoint> {
        let cp: Checkpoint = self.persistence.read_versioned(path)?;
        self.verify_checkpoint(&cp)?;
        Ok(cp)
    }

    pub fn verify_checkpoint(&self, cp: &Checkpoint) -> Result<()> {
        if cp.manifest.checkpoint_hash
            != Self::hash(
                cp.manifest.sequence,
                cp.manifest.journal_position,
                &cp.manifest.state_root,
                &cp.state_snapshot,
            )
        {
            return Err(anyhow!("checkpoint integrity verification failed"));
        }
        let root = hex::encode(Sha256::digest(&cp.state_snapshot));
        if root != cp.manifest.state_root {
            return Err(anyhow!("checkpoint root validation failed"));
        }
        Ok(())
    }

    fn path_for(&self, sequence: u64) -> PathBuf {
        self.dir.join(format!("checkpoint-{sequence:020}.json"))
    }
    fn hash(sequence: u64, journal_position: u64, state_root: &str, snapshot: &[u8]) -> String {
        let mut h = Sha256::new();
        h.update(sequence.to_le_bytes());
        h.update(journal_position.to_le_bytes());
        h.update(state_root.as_bytes());
        h.update(snapshot);
        hex::encode(h.finalize())
    }
}
