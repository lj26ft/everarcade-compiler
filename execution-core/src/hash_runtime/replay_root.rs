use crate::hash_runtime::canonical_hash::canonical_hash;

pub fn replay_root(prior_root: &str, receipt_hash: &str, next_root: &str, logical_index: usize) -> String {
    canonical_hash(
        format!("{prior_root}:{receipt_hash}:{next_root}:{logical_index}").as_bytes(),
    )
}
