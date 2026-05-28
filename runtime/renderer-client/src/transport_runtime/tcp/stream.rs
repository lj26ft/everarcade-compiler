use crate::transport_runtime::wire::ReplayChunkWireMessage;
use std::collections::HashSet;

#[derive(Debug, Clone, Default)]
pub struct TcpReplayStream {
    seen: HashSet<u64>,
    pub next_sequence: u64,
    pub continuity_root: String,
    pub accepted: Vec<ReplayChunkWireMessage>,
}

impl TcpReplayStream {
    pub fn with_root(root: impl Into<String>) -> Self {
        Self {
            continuity_root: root.into(),
            ..Self::default()
        }
    }
    pub fn ingest(&mut self, chunk: ReplayChunkWireMessage) -> Result<(), String> {
        chunk.validate()?;
        if chunk.continuity_root != self.continuity_root {
            return Err("continuity_root_divergence_rejected".into());
        }
        if self.seen.contains(&chunk.sequence) {
            return Err("duplicate_replay_chunk".into());
        }
        if chunk.sequence != self.next_sequence {
            return Err("out_of_order_replay_chunk".into());
        }
        self.seen.insert(chunk.sequence);
        self.next_sequence += 1;
        self.accepted.push(chunk);
        Ok(())
    }
}
