use crate::stable_hash;

pub fn inspect_partition_migration(entity_id: &str, from: &str, to: &str) -> String { stable_hash(&["partition-migration", entity_id, from, to]) }
