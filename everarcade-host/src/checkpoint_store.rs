use std::{fs, path::Path};

use crate::error::HostError;

pub fn write_checkpoint_root(
    dir: &Path,
    checkpoint_root: &[u8; 32],
) -> Result<std::path::PathBuf, HostError> {
    let name = format!("{}.bin", hex::encode(checkpoint_root));
    let path = dir.join(name);
    fs::write(&path, checkpoint_root)?;
    Ok(path)
}
