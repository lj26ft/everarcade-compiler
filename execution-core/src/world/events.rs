use crate::{canonical::encoding::canonical_encode, hashing::hash_bytes};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionEvent {
    pub execution_id: String,
    pub partition_id: String,
    pub sequence: u64,
    pub payload: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventChunk {
    pub chunk_index: u64,
    pub events: Vec<ExecutionEvent>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventRoot(pub String);

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventStream {
    pub chunks: Vec<EventChunk>,
}

impl EventStream {
    pub fn canonicalize(&mut self) {
        for chunk in &mut self.chunks {
            chunk.events.sort_by(|a, b| {
                (&a.partition_id, a.sequence, &a.execution_id).cmp(&(
                    &b.partition_id,
                    b.sequence,
                    &b.execution_id,
                ))
            });
        }
        self.chunks.sort_by_key(|c| c.chunk_index);
    }

    pub fn root(&self) -> Result<EventRoot, String> {
        let bytes = canonical_encode(self).map_err(|e| e.to_string())?;
        Ok(EventRoot(hash_bytes(&bytes)))
    }
}
