use crate::{
    error::SdkError,
    game::DeterministicGame,
    input::{deterministic_order, PlayerInput},
    replay::ReplayLog,
    state::GameState,
};

pub struct DeterministicRuntime<G> {
    game: G,
    state: GameState,
    replay: ReplayLog,
    authority: bool,
}
impl<G: DeterministicGame> DeterministicRuntime<G> {
    pub fn new(game: G) -> Self {
        Self {
            game,
            state: GameState::new(),
            replay: ReplayLog::default(),
            authority: true,
        }
    }
    pub fn with_authority(game: G, authority: bool) -> Self {
        Self {
            game,
            state: GameState::new(),
            replay: ReplayLog::default(),
            authority,
        }
    }
    pub fn tick(&mut self, inputs: Vec<PlayerInput>) -> Result<String, SdkError> {
        if !self.authority {
            return Err(SdkError::UnauthorizedAuthorityMutation);
        }
        let ordered = deterministic_order(inputs);
        for input in &ordered {
            self.game.apply_input(&mut self.state, input)?;
        }
        self.replay.append_frame(&self.state, &ordered)?;
        self.state.tick += 1;
        Ok(self.replay.frames().last().unwrap().state_hash.clone())
    }
    pub fn state(&self) -> &GameState {
        &self.state
    }
    pub fn replay(&self) -> &ReplayLog {
        &self.replay
    }
    pub fn replay_mutation_probe(&mut self) -> Result<(), SdkError> {
        self.replay.try_mutate_frame(0, "bad".into())
    }
}
