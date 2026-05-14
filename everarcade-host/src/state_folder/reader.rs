use std::path::{Path, PathBuf};

pub fn artifact_path(base: &Path, area: &str, name: &str) -> PathBuf {
    base.join(area).join(name)
}
