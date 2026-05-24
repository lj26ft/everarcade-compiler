use crate::world::{EconomicLedgerCheckpoint, WorldContinuityRoot};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleCheckpoint {
    pub tick: u64,
    pub entity_count: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldCheckpoint {
    pub tick: u64,
    pub continuity_root: WorldContinuityRoot,
    pub lifecycle: LifecycleCheckpoint,
    pub ledger: EconomicLedgerCheckpoint,
    pub scheduler: SchedulerCheckpoint,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchedulerCheckpoint {
    pub pending_tick_count: usize,
}
