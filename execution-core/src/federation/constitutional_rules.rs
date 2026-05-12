use crate::hashing::hash_bytes;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConstitutionalRules { pub constitution_hash: String, pub version: u64 }

impl ConstitutionalRules {
    pub fn upgrade(&self, patch: &[u8]) -> Self {
        let constitution_hash = hash_bytes(format!("{}:{}", self.constitution_hash, hash_bytes(patch)).as_bytes());
        Self { constitution_hash, version: self.version + 1 }
    }
}
