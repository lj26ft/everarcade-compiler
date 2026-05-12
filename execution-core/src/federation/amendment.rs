use crate::hashing::hash_bytes;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConstitutionalAmendment {
    pub amendment_id: String,
    pub previous_root: String,
    pub next_root: String,
}

impl ConstitutionalAmendment {
    pub fn new(previous_root: impl Into<String>, amendment_payload_hash: impl Into<String>) -> Self {
        let previous_root = previous_root.into();
        let payload_hash = amendment_payload_hash.into();
        let amendment_id = hash_bytes(format!("amendment:{previous_root}:{payload_hash}").as_bytes());
        let next_root = hash_bytes(format!("constitution:{amendment_id}").as_bytes());
        Self { amendment_id, previous_root, next_root }
    }
}
