use sha2::{Digest, Sha256};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameplayState {
    pub tick: u64,
    pub score: u64,
    pub continuity_root: String,
    pub state_root: String,
}

impl GameplayState {
    pub fn genesis(continuity_root: impl Into<String>) -> Self {
        let continuity_root = continuity_root.into();
        let state_root = root_for(0, 0, &continuity_root);
        Self {
            tick: 0,
            score: 0,
            continuity_root,
            state_root,
        }
    }

    pub fn advance(&self, delta: u64) -> Self {
        let tick = self.tick + 1;
        let score = self.score + delta;
        let state_root = root_for(tick, score, &self.continuity_root);
        Self {
            tick,
            score,
            continuity_root: self.continuity_root.clone(),
            state_root,
        }
    }
}

pub fn root_for(tick: u64, score: u64, continuity_root: &str) -> String {
    let mut h = Sha256::new();
    h.update(b"everarcade:gameplay-state:v1");
    h.update(tick.to_be_bytes());
    h.update(score.to_be_bytes());
    h.update(continuity_root.as_bytes());
    format!("sha256:{}", hex::encode(h.finalize()))
}
