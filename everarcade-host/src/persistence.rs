use std::{fs, path::PathBuf};

use crate::error::HostError;

#[derive(Clone, Debug)]
pub struct HostPaths {
    pub root: PathBuf,
    pub packages: PathBuf,
    pub receipts: PathBuf,
    pub checkpoints: PathBuf,
    pub anchors: PathBuf,
    pub manifests: PathBuf,
}

impl HostPaths {
    pub fn new(root: PathBuf) -> Self {
        Self {
            packages: root.join("packages"),
            receipts: root.join("receipts"),
            checkpoints: root.join("checkpoints"),
            anchors: root.join("anchors"),
            manifests: root.join("manifests"),
            root,
        }
    }

    pub fn ensure(&self) -> Result<(), HostError> {
        for d in [&self.root, &self.packages, &self.receipts, &self.checkpoints, &self.anchors, &self.manifests] {
            fs::create_dir_all(d)?;
        }
        Ok(())
    }
}
