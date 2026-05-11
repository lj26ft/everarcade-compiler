#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkNode {
    pub node_id: String,
    pub supported_epochs: Vec<u64>,
    pub execution_capabilities: Vec<String>,
    pub verifier: bool,
    pub archive: bool,
    pub execution: bool,
    pub reputation_score: u64,
}

impl NetworkNode {
    pub fn supports_epoch(&self, epoch: u64) -> bool {
        self.supported_epochs.contains(&epoch)
    }
}
