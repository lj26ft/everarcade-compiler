use sha2::{Digest, Sha256};

use super::{
    execution::AuthorityBoundary,
    replay::{GameplayReplayCheckpoint, GameplayReplayContinuity, GameplayReplayWindow},
    GameplayExecution, GameplayInput, GameplaySession, GameplayWorld,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GameplayRuntimeError {
    UnauthorizedMutation,
    ReplayMutation,
    Divergence,
    ReplayCorruption,
    InvalidSchedule,
    InvalidRestoration,
}

#[derive(Clone, Debug)]
pub struct GameplayRuntime {
    pub session: GameplaySession,
    pub world: GameplayWorld,
    pub windows: Vec<GameplayReplayWindow>,
    pub checkpoints: Vec<GameplayReplayCheckpoint>,
    pub continuity: GameplayReplayContinuity,
}

impl GameplayRuntime {
    pub fn new(session: GameplaySession) -> Self {
        let world = GameplayWorld::new(session.continuity_root.clone());
        let continuity = GameplayReplayContinuity {
            continuity_root: session.continuity_root.clone(),
            latest_tick: 0,
            append_only: true,
        };
        Self {
            session,
            world,
            windows: Vec::new(),
            checkpoints: Vec::new(),
            continuity,
        }
    }

    pub fn execute_tick(
        &mut self,
        input: GameplayInput,
    ) -> Result<GameplayReplayWindow, GameplayRuntimeError> {
        if input.frame != self.world.state.tick + 1 {
            return Err(GameplayRuntimeError::InvalidSchedule);
        }
        let expected = format!("auth:{}:{}", self.session.session_id, input.player_id);
        if input.authority_token != expected {
            return Err(GameplayRuntimeError::UnauthorizedMutation);
        }
        self.world.apply_delta(input.delta);
        let window = self.produce_window();
        self.append_window(window.clone())?;
        self.checkpoints.push(GameplayReplayCheckpoint {
            tick: self.world.state.tick,
            continuity_root: self.session.continuity_root.clone(),
            state_root: self.world.state.state_root.clone(),
        });
        self.continuity.latest_tick = self.world.state.tick;
        Ok(window)
    }

    pub fn execute_authoritative(
        &mut self,
        execution: GameplayExecution,
    ) -> Result<GameplayReplayWindow, GameplayRuntimeError> {
        if execution.boundary != AuthorityBoundary::DeterministicRuntime {
            return Err(GameplayRuntimeError::UnauthorizedMutation);
        }
        if execution.scheduled_tick != self.world.state.tick + 1 {
            return Err(GameplayRuntimeError::InvalidSchedule);
        }
        self.execute_tick(execution.input)
    }

    pub fn append_window(
        &mut self,
        window: GameplayReplayWindow,
    ) -> Result<(), GameplayRuntimeError> {
        let previous_end = self.windows.last().map(|w| w.end_tick).unwrap_or(0);
        super::replay::validation::validate_append_only(previous_end, &window)?;
        if window.continuity_root != self.session.continuity_root
            || window.state_root != self.world.state.state_root
        {
            return Err(GameplayRuntimeError::ReplayCorruption);
        }
        self.windows.push(window);
        Ok(())
    }

    pub fn restore(
        session: GameplaySession,
        checkpoint: GameplayReplayCheckpoint,
        score: u64,
    ) -> Result<Self, GameplayRuntimeError> {
        if checkpoint.continuity_root != session.continuity_root {
            return Err(GameplayRuntimeError::InvalidRestoration);
        }
        let state = super::replay::recovery::restore_checkpoint(&checkpoint, score)?;
        let world = GameplayWorld { state };
        let continuity = GameplayReplayContinuity {
            continuity_root: session.continuity_root.clone(),
            latest_tick: checkpoint.tick,
            append_only: true,
        };
        Ok(Self {
            session,
            world,
            windows: Vec::new(),
            checkpoints: vec![checkpoint],
            continuity,
        })
    }

    pub fn reject_replay_derived_authority(source: &str) -> Result<(), GameplayRuntimeError> {
        if source.contains("replay") || source.contains("renderer") || source.contains("observer") {
            return Err(GameplayRuntimeError::UnauthorizedMutation);
        }
        Ok(())
    }

    fn produce_window(&self) -> GameplayReplayWindow {
        let start_tick = self.world.state.tick - 1;
        let end_tick = self.world.state.tick;
        let manifest_root = manifest_root(start_tick, end_tick, &self.world.state.state_root);
        GameplayReplayWindow {
            start_tick,
            end_tick,
            continuity_root: self.session.continuity_root.clone(),
            state_root: self.world.state.state_root.clone(),
            manifest_root,
        }
    }
}

fn manifest_root(start: u64, end: u64, state_root: &str) -> String {
    let mut h = Sha256::new();
    h.update(b"everarcade:gameplay-replay-window:v1");
    h.update(start.to_be_bytes());
    h.update(end.to_be_bytes());
    h.update(state_root.as_bytes());
    format!("sha256:{}", hex::encode(h.finalize()))
}
