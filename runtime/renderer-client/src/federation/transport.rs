use std::collections::HashSet;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProjectionReplayStreamEnvelope {
    pub stream_id: String,
    pub session_id: String,
    pub sequence: u64,
    pub payload_hash: String,
    pub continuity_hash: String,
    pub replay_window_root: String,
}

#[derive(Default)]
pub struct ReplayTransportGuard {
    seen: HashSet<(String, u64)>,
    next_seq: u64,
}

impl ReplayTransportGuard {
    pub fn accept(&mut self, env: &ProjectionReplayStreamEnvelope) -> Result<(), String> {
        let key = (env.stream_id.clone(), env.sequence);
        if self.seen.contains(&key) {
            return Err("duplicate_replay_chunk".to_string());
        }
        if env.sequence != self.next_seq {
            return Err("out_of_order_replay_chunk".to_string());
        }
        if env.replay_window_root.is_empty() || env.continuity_hash.is_empty() {
            return Err("invalid_continuity".to_string());
        }
        self.seen.insert(key);
        self.next_seq += 1;
        Ok(())
    }
}
