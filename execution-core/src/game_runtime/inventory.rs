use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct InventoryState {
    pub ownership: BTreeMap<String, BTreeSet<String>>,
}

impl InventoryState {
    pub fn add_item(&mut self, player_id: &str, item_id: &str) {
        self.ownership
            .entry(player_id.to_string())
            .or_default()
            .insert(item_id.to_string());
    }
}
