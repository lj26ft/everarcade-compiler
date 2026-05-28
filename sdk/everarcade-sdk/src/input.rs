#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PlayerInput {
    pub tick: u64,
    pub player_id: String,
    pub command: String,
}

impl PlayerInput {
    pub fn new(tick: u64, player_id: impl Into<String>, command: impl Into<String>) -> Self {
        Self {
            tick,
            player_id: player_id.into(),
            command: command.into(),
        }
    }
}

pub fn deterministic_order(mut inputs: Vec<PlayerInput>) -> Vec<PlayerInput> {
    inputs.sort();
    inputs
}
