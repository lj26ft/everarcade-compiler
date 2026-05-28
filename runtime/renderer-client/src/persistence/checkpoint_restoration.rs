use crate::persistence::checkpoint::ProjectionReplayCheckpoint;
use crate::persistence::hash;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersistedReplayCheckpoint {
    pub checkpoint: ProjectionReplayCheckpoint,
    pub checkpoint_root: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ReplayCheckpointPersistence;

impl ReplayCheckpointPersistence {
    pub fn persist_checkpoint(
        checkpoint: ProjectionReplayCheckpoint,
    ) -> Result<PersistedReplayCheckpoint, String> {
        let checkpoint_root = hash::hash_serialized(&checkpoint)?;
        Ok(PersistedReplayCheckpoint {
            checkpoint,
            checkpoint_root,
        })
    }

    pub fn restore_checkpoint(
        persisted: &PersistedReplayCheckpoint,
    ) -> Result<ProjectionReplayCheckpoint, String> {
        let expected = hash::hash_serialized(&persisted.checkpoint)?;
        if expected != persisted.checkpoint_root {
            return Err("checkpoint persistence corruption".into());
        }
        Ok(persisted.checkpoint.clone())
    }

    pub fn restore_continuity_root(
        persisted: &PersistedReplayCheckpoint,
    ) -> Result<String, String> {
        Self::restore_checkpoint(persisted).map(|checkpoint| checkpoint.continuity_root)
    }
}
