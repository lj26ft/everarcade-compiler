use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameState {
    pub tick: u64,
    facts: Vec<(String, String)>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            tick: 0,
            facts: Vec::new(),
        }
    }
    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        let key = key.into();
        let value = value.into();
        match self.facts.binary_search_by(|(k, _)| k.cmp(&key)) {
            Ok(idx) => self.facts[idx] = (key, value),
            Err(idx) => self.facts.insert(idx, (key, value)),
        }
    }
    pub fn get(&self, key: &str) -> Option<&str> {
        self.facts
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.as_str())
    }
    pub fn facts(&self) -> &[(String, String)] {
        &self.facts
    }
    pub fn deterministic_hash(&self) -> String {
        let mut h = Sha256::new();
        h.update(self.tick.to_le_bytes());
        for (k, v) in &self.facts {
            h.update(k.as_bytes());
            h.update([0]);
            h.update(v.as_bytes());
            h.update([0xff]);
        }
        hex::encode(h.finalize())
    }
}
impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}
