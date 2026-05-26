use super::input::PlayerInput;
pub fn deterministic_order(mut inputs: Vec<PlayerInput>) -> Vec<PlayerInput> {
    inputs.sort_by(|a, b| a.player_id.cmp(&b.player_id));
    inputs
}
