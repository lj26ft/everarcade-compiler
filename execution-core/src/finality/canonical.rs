use super::checkpoint::FinalityCheckpoint;

#[derive(Debug, Clone, Default)]
pub struct CanonicalHistory {
    checkpoints: Vec<FinalityCheckpoint>,
}

impl CanonicalHistory {
    pub fn append_finalized(&mut self, checkpoint: FinalityCheckpoint) {
        self.checkpoints.push(checkpoint);
    }

    pub fn latest(&self) -> Option<&FinalityCheckpoint> {
        self.checkpoints.last()
    }
}
