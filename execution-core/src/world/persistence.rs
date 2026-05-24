use crate::world::{CivilizationArchive, WorldCheckpoint};

pub fn append_checkpoint(archive: &mut CivilizationArchive, checkpoint: WorldCheckpoint) {
    archive.checkpoints.push(checkpoint);
}
