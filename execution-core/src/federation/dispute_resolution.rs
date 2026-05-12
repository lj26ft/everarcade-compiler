use crate::hashing::hash_bytes;

pub fn dispute_record(claim: &str, evidence_root: &str) -> String {
    hash_bytes(format!("{claim}:{evidence_root}").as_bytes())
}
