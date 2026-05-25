#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MemoryBudget {
    pub max_guest_bytes: u64,
    pub max_host_bytes: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct EventBudget {
    pub max_events: u64,
    pub max_chunk_bytes: u64,
    pub max_archive_growth_bytes: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct WitnessBudget {
    pub max_chunk_bytes: u64,
    pub max_chain_depth: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ReplayBudget {
    pub max_window: u64,
    pub max_restoration_depth: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SnapshotBudget {
    pub max_segment_count: u64,
    pub max_chain_depth: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ExecutionQuota {
    pub max_fuel: u64,
    pub max_mutations: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ResourceBudget {
    pub memory: MemoryBudget,
    pub events: EventBudget,
    pub witness: WitnessBudget,
    pub replay: ReplayBudget,
    pub snapshot: SnapshotBudget,
    pub execution: ExecutionQuota,
}
