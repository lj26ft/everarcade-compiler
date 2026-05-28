use crate::transport_runtime::wire::ReplayCheckpointWireMessage;
use std::collections::BTreeMap;
#[derive(Debug, Clone, Default)]
pub struct LiveReplayCheckpointStore {
    pub checkpoints: BTreeMap<u64, ReplayCheckpointWireMessage>,
}
impl LiveReplayCheckpointStore {
    pub fn append(&mut self, checkpoint: ReplayCheckpointWireMessage) -> Result<(), String> {
        checkpoint.validate()?;
        self.checkpoints
            .insert(checkpoint.checkpoint_sequence, checkpoint);
        Ok(())
    }
    pub fn latest_tip(&self) -> u64 {
        self.checkpoints
            .values()
            .last()
            .map(|c| c.replay_tip)
            .unwrap_or(0)
    }
}
