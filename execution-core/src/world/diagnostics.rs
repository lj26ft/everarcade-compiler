use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersistentWorldDiagnostics {
    pub world_id: String,
    pub civilization_epoch: u64,
    pub tick_index: u64,
    pub entity_count: usize,
    pub inventory_mutations: usize,
    pub economy_mutations: usize,
    pub checkpoint_count: usize,
    pub archive_size_bytes: usize,
    pub restoration_possible: bool,
}
