#![allow(dead_code)]

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TopologyRouter {
    pub deterministic_continuity: bool,
    pub replay_only: bool,
}

impl TopologyRouter {
    pub fn activate() -> Self {
        Self { deterministic_continuity: true, replay_only: true }
    }

    pub fn validate(&self) -> bool {
        self.deterministic_continuity && self.replay_only
    }
}
