use crate::runtime::{CheckpointManager, JournalManager, ReplayManager};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RecoveryReport {
    pub restored_checkpoint: u64,
    pub replayed_entries: u64,
    pub verified_root: String,
    pub status: String,
}

#[derive(Clone, Debug)]
pub struct RecoveryManager {
    pub checkpoint_manager: CheckpointManager,
    pub journal_manager: JournalManager,
    pub replay_manager: ReplayManager,
}

impl RecoveryManager {
    pub fn recover(&self) -> Result<RecoveryReport> {
        let checkpoint = self.checkpoint_manager.latest()?;
        let entries = self.journal_manager.entries()?;
        if let Some(cp) = checkpoint {
            let replay_entries: Vec<_> = entries
                .into_iter()
                .filter(|e| e.sequence > cp.manifest.journal_position)
                .collect();
            let latest_root = replay_entries
                .last()
                .map(|e| e.state_root.clone())
                .unwrap_or_else(|| cp.manifest.state_root.clone());
            let _report = if replay_entries.is_empty() {
                None
            } else {
                Some(
                    self.replay_manager
                        .report(&cp.state_snapshot, &replay_entries, &latest_root),
                )
            };
            Ok(RecoveryReport {
                restored_checkpoint: cp.manifest.sequence,
                replayed_entries: replay_entries.len() as u64,
                verified_root: latest_root,
                status: "recovered".into(),
            })
        } else {
            let root = entries
                .last()
                .map(|e| e.state_root.clone())
                .unwrap_or_default();
            Ok(RecoveryReport {
                restored_checkpoint: 0,
                replayed_entries: entries.len() as u64,
                verified_root: root,
                status: "recovered-from-genesis".into(),
            })
        }
    }
}
