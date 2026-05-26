use serde::{Deserialize, Serialize};

use super::input_runtime::RuntimeInput;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayTickRecord {
    pub tick: u64,
    pub inputs: Vec<RuntimeInput>,
    pub state_root: String,
    pub event_root: String,
    pub validation_root: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayRecord {
    pub ticks: Vec<ReplayTickRecord>,
}

impl ReplayRecord {
    pub fn append_replay(&mut self, tick: ReplayTickRecord) {
        self.ticks.push(tick);
        self.ticks.sort_by_key(|t| t.tick);
    }
    pub fn load_replay(ticks: Vec<ReplayTickRecord>) -> Self {
        Self { ticks }
    }
    pub fn verify_replay(&self) -> bool {
        self.ticks.windows(2).all(|w| w[0].tick <= w[1].tick)
    }
    pub fn resume_replay(&self, from_tick: u64) -> Vec<ReplayTickRecord> {
        self.ticks
            .iter()
            .filter(|t| t.tick >= from_tick)
            .cloned()
            .collect()
    }
}
