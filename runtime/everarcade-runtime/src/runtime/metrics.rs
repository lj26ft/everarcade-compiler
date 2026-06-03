use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RuntimeMetrics {
    pub ticks_executed: u64,
    pub receipts_generated: u64,
    pub journal_size_bytes: u64,
    pub checkpoint_count: u64,
    pub recovery_count: u64,
    pub replay_duration_ms: u128,
    pub input_queue_depth: usize,
    pub memory_usage_bytes: u64,
    pub disk_usage_bytes: u64,
}
