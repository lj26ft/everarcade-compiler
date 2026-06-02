use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EverNodeTopology {
    pub node_count: usize,
    pub validations: Vec<String>,
}

impl EverNodeTopology {
    pub fn new(node_count: usize) -> Result<Self, String> {
        if ![1, 2, 5, 10].contains(&node_count) {
            return Err("unsupported EverNode federation size".into());
        }
        Ok(Self {
            node_count,
            validations: Vec::new(),
        })
    }
    pub fn validate_all(&mut self) {
        self.validations = [
            "node join",
            "node leave",
            "node recovery",
            "checkpoint sync",
            "replay sync",
            "partition recovery",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
    }
    pub fn has_validation(&self, name: &str) -> bool {
        self.validations.iter().any(|v| v == name)
    }
}
