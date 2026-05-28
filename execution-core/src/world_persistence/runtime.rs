use super::{archive::ReplayArchive, checkpoint::WorldCheckpoint, validation::validate_checkpoint};
#[derive(Clone, Debug)]
pub struct WorldPersistenceRuntime {
    pub archive: ReplayArchive,
    pub checkpoints: Vec<WorldCheckpoint>,
}
impl WorldPersistenceRuntime {
    pub fn new() -> Self {
        Self {
            archive: ReplayArchive::new(),
            checkpoints: vec![],
        }
    }
    pub fn archive_replay(&mut self, entry: &str) {
        self.archive.append(entry)
    }
    pub fn persist_checkpoint(&mut self, cp: WorldCheckpoint) -> Result<(), &'static str> {
        if validate_checkpoint(&cp) {
            self.checkpoints.push(cp);
            Ok(())
        } else {
            Err("corrupted checkpoint rejected")
        }
    }
}
