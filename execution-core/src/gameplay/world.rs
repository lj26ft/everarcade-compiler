use super::state::GameplayState;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameplayWorld {
    pub state: GameplayState,
}

impl GameplayWorld {
    pub fn new(continuity_root: impl Into<String>) -> Self {
        Self {
            state: GameplayState::genesis(continuity_root),
        }
    }
    pub fn apply_delta(&mut self, delta: u64) {
        self.state = self.state.advance(delta);
    }
}
