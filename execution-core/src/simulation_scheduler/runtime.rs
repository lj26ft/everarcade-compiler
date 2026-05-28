use serde::{Deserialize, Serialize};

use super::{partition::PartitionWork, priority, validation};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SimulationSchedulerError {
    OrderingDivergence,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ScheduledWork {
    pub partition_id: String,
    pub priority: u64,
    pub tick: u64,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct SimulationSchedulerRuntime {
    pub tick: u64,
    pub schedule: Vec<ScheduledWork>,
}
impl SimulationSchedulerRuntime {
    pub fn schedule(
        &mut self,
        work: Vec<PartitionWork>,
    ) -> Result<Vec<ScheduledWork>, SimulationSchedulerError> {
        let mut out: Vec<_> = work
            .into_iter()
            .map(|w| ScheduledWork {
                priority: priority::priority_score(&w.partition_id, self.tick)
                    + w.ecs_entities
                    + w.ai_entities,
                partition_id: w.partition_id,
                tick: self.tick,
            })
            .collect();
        out.sort_by(|a, b| {
            a.priority
                .cmp(&b.priority)
                .then_with(|| a.partition_id.cmp(&b.partition_id))
        });
        if !validation::schedule_is_deterministic(&out) {
            return Err(SimulationSchedulerError::OrderingDivergence);
        }
        self.schedule.extend(out.clone());
        self.tick += 1;
        Ok(out)
    }
}
