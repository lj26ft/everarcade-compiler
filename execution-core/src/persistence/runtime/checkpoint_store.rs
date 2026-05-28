use crate::gameplay::replay::GameplayReplayCheckpoint;
#[derive(Clone, Debug, Default)]
pub struct RuntimeCheckpointStore {
    pub checkpoints: Vec<GameplayReplayCheckpoint>,
}
impl RuntimeCheckpointStore {
    pub fn persist(&mut self, checkpoint: GameplayReplayCheckpoint) {
        self.checkpoints.push(checkpoint);
    }
    pub fn latest(&self) -> Option<&GameplayReplayCheckpoint> {
        self.checkpoints.last()
    }
}
