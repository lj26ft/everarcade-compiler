use super::checkpoint_delta::CheckpointDelta;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FastSyncPlan {
    pub import_latest_checkpoint: bool,
    pub replay_delta: CheckpointDelta,
}
