use super::runtime::AuthoritativeScheduler;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SchedulerError {
    TickDivergence,
    NonDeterministicExecution,
}

pub fn validate_ordering(scheduler: &AuthoritativeScheduler) -> Result<(), SchedulerError> {
    for pair in scheduler.frames.windows(2) {
        if pair[1].tick != pair[0].tick + 1 {
            return Err(SchedulerError::TickDivergence);
        }
    }
    Ok(())
}
