use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct PlayerId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CharacterId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SessionId(pub String);

fn deterministic_id(prefix: &str, parts: &[&str]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(prefix.as_bytes());
    for part in parts {
        hasher.update([0]);
        hasher.update(part.as_bytes());
    }
    format!("{prefix}-{}", hex::encode(&hasher.finalize()[..8]))
}

impl PlayerId {
    pub fn new(seed: &str) -> Self {
        Self(deterministic_id("player", &[seed]))
    }
}

impl CharacterId {
    pub fn for_player(player_id: &PlayerId) -> Self {
        Self(deterministic_id("character", &[&player_id.0]))
    }
}

impl SessionId {
    pub fn new(game_id: &str, player_id: &PlayerId) -> Self {
        Self(deterministic_id("session", &[game_id, &player_id.0]))
    }
}

impl fmt::Display for PlayerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl fmt::Display for CharacterId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl fmt::Display for SessionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InventoryItem {
    pub item: String,
    pub quantity: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Character {
    pub id: CharacterId,
    pub player_id: PlayerId,
    pub class_name: String,
    pub health: i64,
    pub max_health: i64,
    pub energy: i64,
    pub max_energy: i64,
    pub attack: i64,
    pub defense: i64,
    pub inventory: Vec<InventoryItem>,
    pub level: u64,
    pub experience: u64,
    pub x: i64,
    pub y: i64,
    pub alive: bool,
}

impl Character {
    pub fn starter_warrior(player_id: PlayerId) -> Self {
        Self {
            id: CharacterId::for_player(&player_id),
            player_id,
            class_name: "Warrior".to_owned(),
            health: 100,
            max_health: 100,
            energy: 50,
            max_energy: 50,
            attack: 20,
            defense: 5,
            inventory: vec![InventoryItem {
                item: "gold".to_owned(),
                quantity: 0,
            }],
            level: 1,
            experience: 0,
            x: 0,
            y: 0,
            alive: true,
        }
    }

    pub fn add_item(&mut self, item: &str, quantity: u64) {
        if let Some(existing) = self.inventory.iter_mut().find(|entry| entry.item == item) {
            existing.quantity += quantity;
        } else {
            self.inventory.push(InventoryItem {
                item: item.to_owned(),
                quantity,
            });
        }
    }
}
