use crate::hashing::hash_bytes;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Treaty {
    pub treaty_id: String,
    pub previous_treaty: Option<String>,
    pub terms_hash: String,
}

impl Treaty {
    pub fn genesis(terms: &[u8]) -> Self {
        let terms_hash = hash_bytes(terms);
        let treaty_id = hash_bytes(format!("treaty:{terms_hash}").as_bytes());
        Self {
            treaty_id,
            previous_treaty: None,
            terms_hash,
        }
    }

    pub fn evolve(&self, terms: &[u8]) -> Self {
        let terms_hash = hash_bytes(terms);
        let treaty_id = hash_bytes(format!("treaty:{}:{terms_hash}", self.treaty_id).as_bytes());
        Self {
            treaty_id,
            previous_treaty: Some(self.treaty_id.clone()),
            terms_hash,
        }
    }
}
