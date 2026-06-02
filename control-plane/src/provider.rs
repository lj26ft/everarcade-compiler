use crate::health::RuntimeHealth;
use crate::leases::LeaseResources;
use crate::metrics::MetricsSnapshot;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderLeaseRequest {
    pub game_id: String,
    pub resources: LeaseResources,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderLease {
    pub lease_id: String,
    pub host_id: String,
    pub state: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderPackage {
    pub game_id: String,
    pub package_hash: String,
    pub runtime_version: String,
    pub rustrig_hashes: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderDeployment {
    pub deployment_id: String,
    pub lease_id: String,
    pub runtime_id: String,
    pub steps: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderRuntimeHandle {
    pub runtime_id: String,
    pub lease_id: String,
    pub process_state: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderRecoveryReport {
    pub runtime_id: String,
    pub checkpoint_root: String,
    pub replay_root: String,
    pub continuity_root: String,
    pub rejoined_federation: bool,
}

pub trait RuntimeProvider {
    fn allocate_lease(&mut self, request: ProviderLeaseRequest) -> Result<ProviderLease, String>;
    fn release_lease(&mut self, lease_id: &str) -> Result<(), String>;
    fn deploy_package(
        &mut self,
        lease_id: &str,
        package: ProviderPackage,
    ) -> Result<ProviderDeployment, String>;
    fn start_runtime(&mut self, runtime_id: &str) -> Result<ProviderRuntimeHandle, String>;
    fn stop_runtime(&mut self, runtime_id: &str) -> Result<ProviderRuntimeHandle, String>;
    fn restart_runtime(&mut self, runtime_id: &str) -> Result<ProviderRuntimeHandle, String>;
    fn collect_health(&self, runtime_id: &str) -> Result<RuntimeHealth, String>;
    fn collect_metrics(&self) -> Result<MetricsSnapshot, String>;
    fn perform_recovery(&mut self, runtime_id: &str) -> Result<ProviderRecoveryReport, String>;
}
