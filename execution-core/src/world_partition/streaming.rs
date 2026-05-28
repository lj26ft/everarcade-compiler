use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PartitionStreamWindow {
    pub partition_id: String,
    pub replay_root: String,
    pub sequence: u64,
}
pub fn stream_partition(
    partition_id: &str,
    replay_root: &str,
    sequence: u64,
) -> PartitionStreamWindow {
    PartitionStreamWindow {
        partition_id: partition_id.to_string(),
        replay_root: replay_root.to_string(),
        sequence,
    }
}
