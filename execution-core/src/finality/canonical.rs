use super::checkpoint::FinalizedCheckpoint;

#[derive(Debug, Clone, Default)]
pub struct CanonicalHistory {
    checkpoints: Vec<FinalizedCheckpoint>,
}

impl CanonicalHistory {
    pub fn append_finalized(&mut self, checkpoint: FinalizedCheckpoint) {
        self.checkpoints.push(checkpoint);
    }

    pub fn latest(&self) -> Option<&FinalizedCheckpoint> {
        self.checkpoints.last()
    }
}
