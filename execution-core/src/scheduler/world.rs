use super::{events::ScheduledEvent, tick::DeterministicTick};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorldCheckpoint {
    pub lineage: u64,
    pub tick: DeterministicTick,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TickReceipt {
    pub lineage: u64,
    pub tick: DeterministicTick,
    pub event_sequence: Option<u64>,
}

pub trait DeterministicWorld {
    fn checkpoint(&self) -> WorldCheckpoint;
    fn apply(&mut self, tick: DeterministicTick, event: Option<&ScheduledEvent>) -> TickReceipt;
    fn persist_checkpoint(&mut self, checkpoint: WorldCheckpoint);
}
