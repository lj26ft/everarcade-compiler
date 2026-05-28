use super::{frame::ExecutionFrame, recovery::SchedulerCheckpoint, validation::SchedulerError};
use crate::gameplay::{GameplayExecution, GameplayRuntime, GameplayRuntimeError};

#[derive(Clone, Debug)]
pub struct AuthoritativeScheduler {
    pub next_tick: u64,
    pub frames: Vec<ExecutionFrame>,
    pub checkpoints: Vec<SchedulerCheckpoint>,
}

impl AuthoritativeScheduler {
    pub fn new(next_tick: u64) -> Self {
        Self {
            next_tick,
            frames: Vec::new(),
            checkpoints: Vec::new(),
        }
    }

    pub fn schedule(&mut self, execution: GameplayExecution) -> Result<(), SchedulerError> {
        if execution.scheduled_tick != self.next_tick {
            return Err(SchedulerError::TickDivergence);
        }
        self.frames.push(ExecutionFrame {
            tick: execution.scheduled_tick,
            player_id: execution.input.player_id.clone(),
            input_delta: execution.input.delta,
        });
        self.next_tick += 1;
        Ok(())
    }

    pub fn execute(
        &mut self,
        runtime: &mut GameplayRuntime,
        execution: GameplayExecution,
    ) -> Result<(), GameplayRuntimeError> {
        self.schedule(execution.clone())
            .map_err(|_| GameplayRuntimeError::InvalidSchedule)?;
        runtime.execute_authoritative(execution)?;
        self.checkpoints.push(SchedulerCheckpoint {
            next_tick: self.next_tick,
            continuity_root: runtime.session.continuity_root.clone(),
        });
        Ok(())
    }
}
