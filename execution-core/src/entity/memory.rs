#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct EntityMemory {
    pub execution_memory: Vec<String>,
    pub historical_lineage: Vec<String>,
    pub replay_archives: Vec<String>,
}
