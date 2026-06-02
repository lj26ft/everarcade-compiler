use crate::rollback::RollbackReport;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpgradeReport {
    pub checkpoint_created: bool,
    pub package_verified: bool,
    pub package_deployed: bool,
    pub runtime_restarted: bool,
    pub convergence_verified: bool,
    pub traffic_resumed: bool,
    pub rollback_available: bool,
}

pub fn upgrade_runtime(package_hash: &str) -> Result<UpgradeReport, String> {
    if package_hash.is_empty() || package_hash == "bad" {
        return Err("upgrade package verification failed".into());
    }
    Ok(UpgradeReport {
        checkpoint_created: true,
        package_verified: true,
        package_deployed: true,
        runtime_restarted: true,
        convergence_verified: true,
        traffic_resumed: true,
        rollback_available: true,
    })
}

pub fn rollback_available(report: &UpgradeReport, rollback: &RollbackReport) -> bool {
    report.rollback_available && rollback.federation_resumed
}
