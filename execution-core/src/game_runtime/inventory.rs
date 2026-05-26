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

    pub fn drop_item(&mut self, player_id: &str, item_id: &str) -> bool {
        self.ownership
            .get_mut(player_id)
            .map(|items| items.remove(item_id))
            .unwrap_or(false)
    }

    pub fn inspect(&self, player_id: &str) -> Vec<String> {
        self.ownership
            .get(player_id)
            .map(|s| s.iter().cloned().collect())
            .unwrap_or_default()
    }

    pub fn transfer(&mut self, from: &str, to: &str, item_id: &str) -> bool {
        if !self.drop_item(from, item_id) {
            return false;
        }
        self.add_item(to, item_id);
        true
    }
}
