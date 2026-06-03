use crate::runtime::{BackupManager, PackageLoader, ReplayManager};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpgradeReport {
    pub backup_id: String,
    pub promoted_package_hash: Option<String>,
    pub rolled_back: bool,
    pub status: String,
}

#[derive(Clone, Debug)]
pub struct UpgradeManager {
    pub backup: BackupManager,
    pub loader: PackageLoader,
}

impl UpgradeManager {
    pub fn upgrade(
        &self,
        package_path: PathBuf,
        expected_replay_root: Option<String>,
    ) -> Result<UpgradeReport> {
        let backup = self.backup.backup()?;
        let loaded = match self.loader.load(package_path) {
            Ok(pkg) => pkg,
            Err(e) => {
                return Ok(UpgradeReport {
                    backup_id: backup.backup_id,
                    promoted_package_hash: None,
                    rolled_back: true,
                    status: format!("rollback: {e}"),
                })
            }
        };
        if let Some(root) = expected_replay_root {
            let actual = ReplayManager::replay_root(&[], &[]);
            if root != actual {
                return Ok(UpgradeReport {
                    backup_id: backup.backup_id,
                    promoted_package_hash: None,
                    rolled_back: true,
                    status: "rollback: replay verification failed".into(),
                });
            }
        }
        Ok(UpgradeReport {
            backup_id: backup.backup_id,
            promoted_package_hash: Some(loaded.package_hash),
            rolled_back: false,
            status: "promoted".into(),
        })
    }

    pub fn validate_version(&self, from: &str, to: &str) -> Result<()> {
        if from == to {
            return Err(anyhow!("upgrade target must differ from current version"));
        }
        Ok(())
    }
}
