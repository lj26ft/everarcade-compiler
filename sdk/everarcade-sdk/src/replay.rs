use crate::{error::SdkError, input::PlayerInput, state::GameState};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayFrame {
    pub tick: u64,
    pub state_hash: String,
    pub input_count: usize,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ReplayLog {
    frames: Vec<ReplayFrame>,
}

impl ReplayLog {
    pub fn append_frame(
        &mut self,
        state: &GameState,
        inputs: &[PlayerInput],
    ) -> Result<(), SdkError> {
        let next = self.frames.last().map(|f| f.tick + 1).unwrap_or(0);
        if state.tick != next {
            return Err(SdkError::InvalidRuntimeConfiguration(format!(
                "expected replay tick {next}, got {}",
                state.tick
            )));
        }
        self.frames.push(ReplayFrame {
            tick: state.tick,
            state_hash: state.deterministic_hash(),
            input_count: inputs.len(),
        });
        Ok(())
    }
    pub fn frames(&self) -> &[ReplayFrame] {
        &self.frames
    }
    pub fn try_mutate_frame(&mut self, _tick: u64, _hash: String) -> Result<(), SdkError> {
        Err(SdkError::ReplayMutationRejected)
    }
    pub fn continuity_hash(&self) -> String {
        crate::build_game_package(format!("{:?}", self.frames).as_bytes())
    }
}
