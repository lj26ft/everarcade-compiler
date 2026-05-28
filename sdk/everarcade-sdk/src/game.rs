use crate::{error::SdkError, input::PlayerInput, state::GameState};

pub trait DeterministicGame {
    fn game_id(&self) -> &'static str;
    fn apply_input(&self, state: &mut GameState, input: &PlayerInput) -> Result<(), SdkError>;
}

#[derive(Default)]
pub struct CounterGame;
impl DeterministicGame for CounterGame {
    fn game_id(&self) -> &'static str {
        "everarcade.counter"
    }
    fn apply_input(&self, state: &mut GameState, input: &PlayerInput) -> Result<(), SdkError> {
        if input.command.contains("random") || input.command.contains("clock") {
            return Err(SdkError::NonDeterministicMutation);
        }
        let key = format!("player:{}", input.player_id);
        let current = state
            .get(&key)
            .and_then(|v| v.parse::<i64>().ok())
            .unwrap_or(0);
        let delta = match input.command.as_str() {
            "inc" => 1,
            "dec" => -1,
            _ => 0,
        };
        state.set(key, (current + delta).to_string());
        Ok(())
    }
}
