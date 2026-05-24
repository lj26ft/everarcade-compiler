use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeterministicTick(pub u64);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScheduledOperation {
    pub tick: DeterministicTick,
    pub operation_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldScheduler {
    queue: BTreeMap<u64, Vec<String>>,
}

impl WorldScheduler {
    pub fn schedule(&mut self, tick: u64, operation_id: impl Into<String>) {
        self.queue
            .entry(tick)
            .or_default()
            .push(operation_id.into());
    }

    pub fn pop_tick(&mut self, tick: u64) -> Vec<ScheduledOperation> {
        self.queue
            .remove(&tick)
            .unwrap_or_default()
            .into_iter()
            .map(|operation_id| ScheduledOperation {
                tick: DeterministicTick(tick),
                operation_id,
            })
            .collect()
    }
}
