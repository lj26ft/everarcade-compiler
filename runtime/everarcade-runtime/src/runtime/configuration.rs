use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

pub const RUNTIME_VERSION: &str = "everarcade-runtime-v0.1";
pub const LAYOUT_VERSION: u32 = 1;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RuntimeConfiguration {
    pub root: PathBuf,
    pub world_id: String,
    pub package_path: PathBuf,
    pub checkpoint_interval_ticks: u64,
    pub runtime_version: String,
}

impl RuntimeConfiguration {
    pub fn new(
        root: impl AsRef<Path>,
        world_id: impl Into<String>,
        package_path: impl AsRef<Path>,
    ) -> Self {
        Self {
            root: root.as_ref().to_path_buf(),
            world_id: world_id.into(),
            package_path: package_path.as_ref().to_path_buf(),
            checkpoint_interval_ticks: 10,
            runtime_version: RUNTIME_VERSION.to_string(),
        }
    }

    pub fn world_dir(&self) -> PathBuf {
        self.root.join("worlds").join(&self.world_id)
    }
    pub fn packages_dir(&self) -> PathBuf {
        self.root.join("packages")
    }
    pub fn state_dir(&self) -> PathBuf {
        self.world_dir().join("state")
    }
    pub fn journals_dir(&self) -> PathBuf {
        self.world_dir().join("journals")
    }
    pub fn checkpoints_dir(&self) -> PathBuf {
        self.world_dir().join("checkpoints")
    }
    pub fn receipts_dir(&self) -> PathBuf {
        self.world_dir().join("receipts")
    }
    pub fn sessions_dir(&self) -> PathBuf {
        self.world_dir().join("sessions")
    }
    pub fn projections_dir(&self) -> PathBuf {
        self.world_dir().join("projections")
    }
    pub fn reports_dir(&self) -> PathBuf {
        self.root.join("reports")
    }
    pub fn backups_dir(&self) -> PathBuf {
        self.world_dir().join("backups")
    }
    pub fn runtime_status_path(&self) -> PathBuf {
        self.world_dir().join("runtime.json")
    }
}
