use std::path::{Path, PathBuf};

pub fn data_root(base: &Path) -> PathBuf {
    base.join("data")
}
pub fn packages_dir(base: &Path) -> PathBuf {
    data_root(base).join("packages")
}
pub fn receipts_dir(base: &Path) -> PathBuf {
    data_root(base).join("receipts")
}
pub fn checkpoints_dir(base: &Path) -> PathBuf {
    data_root(base).join("checkpoints")
}
pub fn worlds_dir(base: &Path) -> PathBuf {
    data_root(base).join("worlds")
}
