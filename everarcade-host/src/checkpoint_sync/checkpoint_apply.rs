use std::{fs, path::Path};

use super::checkpoint_export::TransferCheckpoint;

pub fn apply_checkpoint(state: &Path, checkpoint: &TransferCheckpoint) -> std::io::Result<()> {
    fs::create_dir_all(state.join("checkpoints"))?;
    let p = state
        .join("checkpoints")
        .join(format!("{}.bin", hex::encode(checkpoint.checkpoint_root)));
    fs::write(p, &checkpoint.state_bytes)
}
