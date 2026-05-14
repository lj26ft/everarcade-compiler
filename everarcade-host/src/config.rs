use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct HostConfig {
    pub package_path: PathBuf,
    pub data_dir: PathBuf,
}

impl HostConfig {
    pub fn new(package_path: impl Into<PathBuf>, data_dir: impl Into<PathBuf>) -> Self {
        Self {
            package_path: package_path.into(),
            data_dir: data_dir.into(),
        }
    }
}
