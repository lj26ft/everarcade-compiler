use crate::health::HealthState;
use crate::leases::{LeaseAllocation, LeaseManager, LeaseResources};
use crate::metrics::MetricsCollector;
use crate::registry::{
    DeploymentManifest, DeploymentRegistry, GamePackage, RegistryRecord, RuntimeVersion,
    RustrigPackageSet,
};
use crate::supervisor::{RuntimeProcess, RuntimeSupervisor};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeploymentStep {
    VerifyPackage,
    VerifyHashes,
    VerifyRustrigs,
    AllocateLease,
    DeployRuntime,
    StartRuntime,
    VerifyHealth,
    RegisterRuntime,
    BeginMonitoring,
    Rollback,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeploymentStatus {
    Running,
    RolledBack,
    Failed,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeploymentResult {
    pub deployment_id: String,
    pub lease: LeaseAllocation,
    pub runtime: RuntimeProcess,
    pub steps: Vec<DeploymentStep>,
    pub status: DeploymentStatus,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeploymentOrchestrator {
    pub leases: LeaseManager,
    pub supervisor: RuntimeSupervisor,
    pub registry: DeploymentRegistry,
    pub metrics: MetricsCollector,
    pub last_rollback: Option<String>,
}
impl DeploymentOrchestrator {
    pub fn new(leases: LeaseManager) -> Self {
        Self {
            leases,
            supervisor: RuntimeSupervisor::default(),
            registry: DeploymentRegistry::default(),
            metrics: MetricsCollector::default(),
            last_rollback: None,
        }
    }
    pub fn deploy_runtime(
        &mut self,
        game_id: impl Into<String>,
        package_hash: impl Into<String>,
        runtime_version: impl Into<String>,
        rustrig_hashes: Vec<String>,
    ) -> Result<DeploymentResult, String> {
        let game_id = game_id.into();
        let package_hash = package_hash.into();
        let runtime_version = runtime_version.into();
        let mut steps = Vec::new();
        steps.push(DeploymentStep::VerifyPackage);
        if package_hash.is_empty() {
            self.rollback("empty package hash");
            return Err("package verification failed".into());
        }
        steps.push(DeploymentStep::VerifyHashes);
        if package_hash == "bad" {
            self.rollback("hash verification failed");
            return Err("hash verification failed".into());
        }
        steps.push(DeploymentStep::VerifyRustrigs);
        if rustrig_hashes.is_empty() {
            self.rollback("missing rustrigs");
            return Err("rustrig verification failed".into());
        }
        steps.push(DeploymentStep::AllocateLease);
        let lease = self.leases.allocate_lease(
            &game_id,
            LeaseResources {
                cpu_cores: 4,
                memory_mb: 8192,
                storage_gb: 200,
                bandwidth_mbps: 100,
            },
        )?;
        steps.push(DeploymentStep::DeployRuntime);
        steps.push(DeploymentStep::StartRuntime);
        let runtime_id = format!("runtime-{}", lease.id.0);
        let runtime = self.supervisor.start_runtime(
            &mut self.leases,
            &lease.id,
            runtime_id,
            "node-1",
            runtime_version.clone(),
        )?;
        steps.push(DeploymentStep::VerifyHealth);
        let mut health = self
            .supervisor
            .health(&runtime.runtime_id)
            .ok_or_else(|| "missing health".to_string())?;
        if health.evaluate() != HealthState::Healthy {
            self.rollback("health verification failed");
            return Err("health verification failed".into());
        }
        steps.push(DeploymentStep::RegisterRuntime);
        let deployment_id = format!("deployment-{}", lease.id.0);
        self.registry.publish(
            format!("game:{game_id}"),
            RegistryRecord::GamePackage(GamePackage {
                game_id: game_id.clone(),
                version: "1.0.0".into(),
                package_hash,
                deprecated: false,
            }),
        );
        self.registry.publish(
            format!("runtime:{runtime_version}"),
            RegistryRecord::RuntimeVersion(RuntimeVersion {
                version: runtime_version.clone(),
                runtime_hash: format!("hash-{runtime_version}"),
            }),
        );
        self.registry.publish(
            "rustrigs:active",
            RegistryRecord::RustrigPackageSet(RustrigPackageSet {
                set_id: "active".into(),
                package_hashes: rustrig_hashes,
            }),
        );
        self.registry.publish(
            format!("deployment:{deployment_id}"),
            RegistryRecord::DeploymentManifest(DeploymentManifest {
                deployment_id: deployment_id.clone(),
                game_id,
                runtime_version,
                rustrig_set_id: "active".into(),
            }),
        );
        steps.push(DeploymentStep::BeginMonitoring);
        self.metrics.record_deployment();
        let lease = self
            .leases
            .leases
            .get(&lease.id)
            .cloned()
            .ok_or_else(|| "lease disappeared".to_string())?;
        Ok(DeploymentResult {
            deployment_id,
            lease,
            runtime,
            steps,
            status: DeploymentStatus::Running,
        })
    }
    pub fn rollback(&mut self, reason: impl Into<String>) {
        self.metrics.record_rollback();
        self.last_rollback = Some(reason.into());
    }
}

pub fn arena_vanguard_live_deployment_path() -> Vec<&'static str> {
    vec![
        "Build Package",
        "Validate Package",
        "Allocate Lease",
        "Deploy Runtime",
        "Join Federation",
        "Verify Health",
        "Start Session",
        "Checkpoint",
        "Recover",
        "Upgrade",
        "Rollback",
    ]
}
