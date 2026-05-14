use super::layout::required_paths;
use std::{fs, path::Path};

pub fn initialize(base: &Path) -> std::io::Result<()> {
    for p in required_paths(base) {
        fs::create_dir_all(p)?;
    }
    Ok(())
}
