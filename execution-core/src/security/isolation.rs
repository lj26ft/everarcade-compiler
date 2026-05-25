#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MemoryIsolationBoundary {
    pub guest_memory_limit_bytes: u64,
    pub host_memory_limit_bytes: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ExecutionIsolationBoundary {
    pub max_fuel: u64,
    pub mutation_limit: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct EventIsolationBoundary {
    pub max_events_per_execution: u64,
    pub max_event_chunk_size_bytes: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RestorationIsolationBoundary {
    pub max_replay_window: u64,
    pub max_restoration_depth: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RuntimeIsolationPolicy {
    pub memory: MemoryIsolationBoundary,
    pub execution: ExecutionIsolationBoundary,
    pub events: EventIsolationBoundary,
    pub restoration: RestorationIsolationBoundary,
}

impl RuntimeIsolationPolicy {
    pub fn deterministic_default() -> Self {
        Self {
            memory: MemoryIsolationBoundary {
                guest_memory_limit_bytes: 1024 * 1024,
                host_memory_limit_bytes: 8 * 1024 * 1024,
            },
            execution: ExecutionIsolationBoundary {
                max_fuel: 1_000_000,
                mutation_limit: 10_000,
            },
            events: EventIsolationBoundary {
                max_events_per_execution: 1024,
                max_event_chunk_size_bytes: 16 * 1024,
            },
            restoration: RestorationIsolationBoundary {
                max_replay_window: 2048,
                max_restoration_depth: 128,
            },
        }
    }
}
