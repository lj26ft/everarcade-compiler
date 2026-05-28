use super::{
    LiveReplayCheckpointStore, LiveReplayChunkStore, LiveReplayIndex, LiveReplayWindowStore,
};
use crate::transport_runtime::wire::{
    ReplayCheckpointWireMessage, ReplayChunkWireMessage, ReplayWindowWireMessage,
};
#[derive(Debug, Clone)]
pub struct LiveReplayStore {
    pub chunks: LiveReplayChunkStore,
    pub windows: LiveReplayWindowStore,
    pub checkpoints: LiveReplayCheckpointStore,
    pub index: LiveReplayIndex,
}
impl LiveReplayStore {
    pub fn open(continuity_root: impl Into<String>) -> Result<Self, String> {
        let root = continuity_root.into();
        Ok(Self {
            chunks: LiveReplayChunkStore::with_root(root.clone()),
            windows: LiveReplayWindowStore::default(),
            checkpoints: LiveReplayCheckpointStore::default(),
            index: LiveReplayIndex::restore(0, root)?,
        })
    }
    pub fn persist_chunk(&mut self, chunk: ReplayChunkWireMessage) -> Result<(), String> {
        self.chunks.append(chunk)?;
        self.index.replay_tip = self.chunks.tip();
        Ok(())
    }
    pub fn persist_window(&mut self, window: ReplayWindowWireMessage) -> Result<(), String> {
        self.windows.append(window)
    }
    pub fn persist_checkpoint(
        &mut self,
        checkpoint: ReplayCheckpointWireMessage,
    ) -> Result<(), String> {
        self.checkpoints.append(checkpoint)?;
        self.index.replay_tip = self.checkpoints.latest_tip().max(self.index.replay_tip);
        Ok(())
    }
}
