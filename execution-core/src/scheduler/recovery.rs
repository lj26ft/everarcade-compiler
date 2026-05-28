use super::runtime::AuthoritativeScheduler;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SchedulerCheckpoint {
    pub next_tick: u64,
    pub continuity_root: String,
}

pub fn recover_schedule(checkpoint: &SchedulerCheckpoint) -> AuthoritativeScheduler {
    AuthoritativeScheduler::new(checkpoint.next_tick)
}
