use crate::runtime::persistence::PersistenceManager;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::VecDeque;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeInput {
    pub sequence: u64,
    pub timestamp_ms: u128,
    pub origin: String,
    pub payload: Vec<u8>,
    pub payload_hash: String,
    pub input_id: String,
}

#[derive(Clone, Debug)]
pub struct InputQueue {
    path: PathBuf,
    persistence: PersistenceManager,
    next_sequence: u64,
    queue: VecDeque<RuntimeInput>,
}

impl InputQueue {
    pub fn open(path: PathBuf, persistence: PersistenceManager) -> Result<Self> {
        let mut queue = VecDeque::new();
        let data = persistence.read_to_string_if_exists(&path)?;
        let mut next_sequence = 1;
        for line in data.lines().filter(|l| !l.trim().is_empty()) {
            let input: RuntimeInput = serde_json::from_str(line)?;
            next_sequence = next_sequence.max(input.sequence + 1);
            queue.push_back(input);
        }
        Ok(Self {
            path,
            persistence,
            next_sequence,
            queue,
        })
    }

    pub fn enqueue(&mut self, origin: impl Into<String>, payload: Vec<u8>) -> Result<RuntimeInput> {
        let payload_hash = hex::encode(Sha256::digest(&payload));
        let sequence = self.next_sequence;
        self.next_sequence += 1;
        let timestamp_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        let input_id = hex::encode(Sha256::digest(
            format!("{sequence}:{timestamp_ms}:{payload_hash}").as_bytes(),
        ));
        let input = RuntimeInput {
            sequence,
            timestamp_ms,
            origin: origin.into(),
            payload,
            payload_hash,
            input_id,
        };
        self.persistence
            .append_line_fsync(&self.path, &serde_json::to_string(&input)?)?;
        self.queue.push_back(input.clone());
        Ok(input)
    }

    pub fn pop(&mut self) -> Option<RuntimeInput> {
        self.queue.pop_front()
    }
    pub fn depth(&self) -> usize {
        self.queue.len()
    }
}
