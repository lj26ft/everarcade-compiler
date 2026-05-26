use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Ord, PartialOrd)]
pub enum InputAction {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Interact,
    InventoryAction,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Ord, PartialOrd)]
pub struct RuntimeInput {
    pub tick: u64,
    pub player_id: String,
    pub action: InputAction,
}

pub fn canonicalize_inputs(inputs: &mut [RuntimeInput]) {
    inputs.sort();
}
