use crate::hashing::hash_bytes;

pub fn lineage_root(ancestors: &[String]) -> String { hash_bytes(ancestors.join("->").as_bytes()) }
