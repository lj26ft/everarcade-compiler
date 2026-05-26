use std::collections::HashSet;

use super::chunk::ReplayChunk;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ReplayTransportCursor {
    pub next_sequence: u64,
    pub last_continuity_hash: String,
}

#[derive(Debug, Clone, Default)]
pub struct ReplayTransportStream {
    seen_sequences: HashSet<u64>,
    pub cursor: ReplayTransportCursor,
    pub accepted: Vec<ReplayChunk>,
}

impl ReplayTransportStream {
    pub fn ingest(&mut self, chunk: ReplayChunk) -> Result<(), String> {
        if self.seen_sequences.contains(&chunk.sequence) {
            return Err("duplicate_replay_chunk".to_string());
        }
        if chunk.sequence != self.cursor.next_sequence {
            return Err("out_of_order_replay_chunk".to_string());
        }
        if self.cursor.next_sequence > 0 && chunk.continuity.previous_hash != self.cursor.last_continuity_hash {
            return Err("invalid_continuity_chain".to_string());
        }
        self.cursor.last_continuity_hash = chunk.continuity.continuity_hash.clone();
        self.cursor.next_sequence += 1;
        self.seen_sequences.insert(chunk.sequence);
        self.accepted.push(chunk);
        Ok(())
    }
}
