use crate::hashing::hash_bytes;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContinuityRecord {
    pub replay_hash: String,
    pub proof_hash: String,
    pub upgrade_hash: String,
    pub archival_hash: String,
    pub continuity_root: String,
}

pub fn continuity_root(replay: &str, proof: &str, upgrade: &str, archival: &str) -> String {
    hash_bytes(format!("{replay}:{proof}:{upgrade}:{archival}").as_bytes())
}
