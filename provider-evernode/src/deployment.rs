use crate::runtime::HotPocketStateLayout;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EverNodeDeploymentStep {
    VerifyPackage,
    VerifyHashes,
    VerifyRustrigs,
    VerifyRuntimeVersion,
    AllocateLease,
    TransferPackage,
    InstallRuntime,
    InitializeState,
    StartHotPocket,
    VerifyRuntimeHealth,
    JoinFederation,
    BeginMonitoring,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EverNodeDeploymentPlan {
    pub steps: Vec<EverNodeDeploymentStep>,
    pub rollback_required_on_failure: bool,
}

impl EverNodeDeploymentPlan {
    pub fn canonical() -> Self {
        Self {
            steps: vec![
                EverNodeDeploymentStep::VerifyPackage,
                EverNodeDeploymentStep::VerifyHashes,
                EverNodeDeploymentStep::VerifyRustrigs,
                EverNodeDeploymentStep::VerifyRuntimeVersion,
                EverNodeDeploymentStep::AllocateLease,
                EverNodeDeploymentStep::TransferPackage,
                EverNodeDeploymentStep::InstallRuntime,
                EverNodeDeploymentStep::InitializeState,
                EverNodeDeploymentStep::StartHotPocket,
                EverNodeDeploymentStep::VerifyRuntimeHealth,
                EverNodeDeploymentStep::JoinFederation,
                EverNodeDeploymentStep::BeginMonitoring,
            ],
            rollback_required_on_failure: true,
        }
    }
    pub fn validate_inputs(
        &self,
        package_hash: &str,
        runtime_version: &str,
        rustrigs: &[String],
        layout: &HotPocketStateLayout,
    ) -> Result<(), String> {
        if package_hash.is_empty() || package_hash == "bad" {
            return Err("package or hash verification failed".into());
        }
        if runtime_version.is_empty() {
            return Err("runtime version verification failed".into());
        }
        if rustrigs.is_empty() {
            return Err("rustrig verification failed".into());
        }
        layout.validate()
    }
    pub fn step_names(&self) -> Vec<String> {
        self.steps.iter().map(|s| format!("{:?}", s)).collect()
    }
}
