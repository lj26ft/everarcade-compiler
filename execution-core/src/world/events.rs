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
pub struct EventStreamCursor {
    pub next_segment: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventWindow {
    pub start_sequence: u64,
    pub end_sequence: u64,
    pub partition_count: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventSegment {
    pub segment_id: u64,
    pub window: EventWindow,
    pub segment_root: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventReplayAnchor {
    pub segment_id: u64,
    pub root: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct StreamingEventArchive {
    pub cursor: EventStreamCursor,
    pub segments: Vec<EventSegment>,
}

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

impl EventSegment {
    pub fn from_window(window: EventWindow, entropy: String) -> Result<Self, String> {
        let segment_id = window.end_sequence;
        let bytes = canonical_encode(&(segment_id, &window, entropy)).map_err(|e| e.to_string())?;
        Ok(Self {
            segment_id,
            window,
            segment_root: hash_bytes(&bytes),
        })
    }
}

impl StreamingEventArchive {
    pub fn push_segment(&mut self, segment: EventSegment) {
        self.cursor.next_segment = segment.segment_id + 1;
        self.segments.push(segment);
        self.segments.sort_by_key(|s| s.segment_id);
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectedRuntimeEvent {
    pub tick: u64,
    pub event_root: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectedWorldState {
    pub tick: u64,
    pub world_root: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectedInventoryState {
    pub inventory_root: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectedEntityState {
    pub entity_root: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectedFrameEvent {
    pub frame_root: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectionReplayEvent {
    pub replay_root: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectionCheckpointEvent {
    pub checkpoint_root: String,
}
