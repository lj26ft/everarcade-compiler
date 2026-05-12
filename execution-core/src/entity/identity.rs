use crate::hashing::hash_bytes;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntityIdentity {
    pub entity_id: String,
    pub genesis_hash: String,
    pub lineage_root: String,
    pub continuity_hash: String,
}

impl EntityIdentity {
    pub fn from_genesis(seed: &[u8]) -> Self {
        let genesis_hash = hash_bytes(seed);
        let entity_id = hash_bytes(format!("entity:{genesis_hash}").as_bytes());
        let lineage_root = hash_bytes(format!("lineage:{entity_id}").as_bytes());
        let continuity_hash = hash_bytes(format!("continuity:{genesis_hash}:{lineage_root}").as_bytes());
        Self { entity_id, genesis_hash, lineage_root, continuity_hash }
    }
}
