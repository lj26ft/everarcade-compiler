use crate::{canonical::encoding::canonical_encode, hashing::hash_bytes};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InventoryState {
    pub ownership: BTreeMap<String, String>,
}

impl InventoryState {
    pub fn apply_transfer(
        &mut self,
        asset_id: &str,
        expected_owner: &str,
        next_owner: &str,
    ) -> Result<(), String> {
        match self.ownership.get(asset_id) {
            Some(owner) if owner == expected_owner => {
                self.ownership
                    .insert(asset_id.to_string(), next_owner.to_string());
                Ok(())
            }
            _ => Err("ambiguous ownership transition".into()),
        }
    }

    pub fn manifest_hash(&self) -> Result<String, String> {
        Ok(hash_bytes(
            &canonical_encode(self).map_err(|e| e.to_string())?,
        ))
    }
}
