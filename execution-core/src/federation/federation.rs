use crate::hashing::hash_bytes;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Federation {
    pub federation_id: String,
    pub member_entities: Vec<String>,
    pub federation_root: String,
    pub continuity_root: String,
}

impl Federation {
    pub fn new(members: Vec<String>) -> Self {
        let mut canonical = members;
        canonical.sort();
        canonical.dedup();
        let federation_root = hash_bytes(canonical.join("|").as_bytes());
        let federation_id = hash_bytes(format!("federation:{federation_root}").as_bytes());
        let continuity_root = hash_bytes(format!("continuity:{federation_id}").as_bytes());
        Self { federation_id, member_entities: canonical, federation_root, continuity_root }
    }
}
