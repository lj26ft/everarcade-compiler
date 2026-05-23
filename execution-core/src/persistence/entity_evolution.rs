use crate::{canonical::encoding::canonical_encode, hashing::hash_bytes};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntityEvolutionRecord {
    pub sequence: u64,
    pub entity_id: String,
    pub prior_state_root: String,
    pub current_state_root: String,
    pub migration_root: String,
    pub previous_hash: String,
}

impl EntityEvolutionRecord {
    pub fn canonical_hash(&self) -> Result<String, String> {
        Ok(hash_bytes(
            &canonical_encode(self).map_err(|e| e.to_string())?,
        ))
    }
}

pub fn evolution_root(records: &[EntityEvolutionRecord]) -> Result<String, String> {
    let mut previous = String::new();
    for record in records {
        if record.previous_hash != previous {
            return Err("entity evolution lineage mismatch".into());
        }
        previous = record.canonical_hash()?;
    }
    Ok(previous)
}
