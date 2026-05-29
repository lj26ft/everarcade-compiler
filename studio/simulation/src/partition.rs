pub fn partition_activity(partition_id: &str, root: &str) -> String { crate::stable_hash(&["partition-activity", partition_id, root]) }
