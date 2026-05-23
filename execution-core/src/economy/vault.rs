use crate::{canonical::encoding::canonical_encode, hashing::hash_bytes};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VaultOwnershipRecord {
    pub sequence: u64,
    pub owner: String,
    pub settlement_hash: String,
    pub previous_record_hash: String,
}

impl VaultOwnershipRecord {
    pub fn canonical_hash(&self) -> Result<String, String> {
        Ok(hash_bytes(
            &canonical_encode(self).map_err(|e| e.to_string())?,
        ))
    }
}

pub fn vault_manifest_hash(records: &[VaultOwnershipRecord]) -> Result<String, String> {
    let mut previous = String::new();
    for record in records {
        if record.previous_record_hash != previous {
            return Err("vault ownership lineage mismatch".into());
        }
        previous = record.canonical_hash()?;
    }
    Ok(previous)
}
