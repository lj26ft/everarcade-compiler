use crate::deployment::EverNodeDeploymentPlan;
use crate::health::collect_runtime_health;
use crate::host::HostBootstrapState;
use crate::lease::{EverNodeLease, EverNodeLeaseState};
use crate::logs::append_provider_log;
use crate::recovery::{recover_runtime, RecoveryRoots};
use crate::runtime::{HotPocketRuntimeConfig, HotPocketStateLayout, RuntimeLifecycle};
use control_plane::health::RuntimeHealth;
use control_plane::logs::{LogKind, LogStore};
use control_plane::metrics::{MetricsCollector, MetricsSnapshot};
use control_plane::provider::{
    ProviderDeployment, ProviderLease, ProviderLeaseRequest, ProviderPackage,
    ProviderRecoveryReport, ProviderRuntimeHandle, RuntimeProvider,
};
use std::collections::BTreeMap;

pub struct EverNodeProvider {
    pub host: HostBootstrapState,
    pub leases: BTreeMap<String, EverNodeLease>,
    pub runtimes: BTreeMap<String, RuntimeLifecycle>,
    pub logs: LogStore,
    pub metrics: MetricsCollector,
    next_lease: u64,
}

impl Default for EverNodeProvider {
    fn default() -> Self {
        Self::new("evernode-host-1")
    }
}

impl EverNodeProvider {
    pub fn new(host_id: impl Into<String>) -> Self {
        let mut host = HostBootstrapState::new(host_id);
        host.bootstrap();
        Self {
            host,
            leases: BTreeMap::new(),
            runtimes: BTreeMap::new(),
            logs: LogStore::default(),
            metrics: MetricsCollector::default(),
            next_lease: 1,
        }
    }

    pub fn bootstrap_host(&mut self) {
        self.host.bootstrap();
    }

    pub fn deployment_plan(&self) -> EverNodeDeploymentPlan {
        EverNodeDeploymentPlan::canonical()
    }

    fn runtime_handle(&self, runtime_id: &str) -> Result<ProviderRuntimeHandle, String> {
        let runtime = self
            .runtimes
            .get(runtime_id)
            .ok_or_else(|| "runtime not found".to_string())?;
        let lease_id = self
            .leases
            .values()
            .find(|l| l.audit.iter().any(|e| e == runtime_id))
            .map(|l| l.lease_id.clone())
            .unwrap_or_default();
        Ok(ProviderRuntimeHandle {
            runtime_id: runtime.runtime_id.clone(),
            lease_id,
            process_state: runtime.process_state.clone(),
        })
    }
}

impl RuntimeProvider for EverNodeProvider {
    fn allocate_lease(&mut self, request: ProviderLeaseRequest) -> Result<ProviderLease, String> {
        let lease_id = format!("evernode-lease-{:06}", self.next_lease);
        self.next_lease += 1;
        let lease = EverNodeLease::allocate(
            &lease_id,
            &self.host.host_id,
            request.game_id,
            request.resources,
        );
        self.leases.insert(lease_id.clone(), lease);
        append_provider_log(&mut self.logs, 1, LogKind::LeaseEvent, &lease_id);
        Ok(ProviderLease {
            lease_id,
            host_id: self.host.host_id.clone(),
            state: "Allocated".into(),
        })
    }

    fn release_lease(&mut self, lease_id: &str) -> Result<(), String> {
        let lease = self
            .leases
            .get_mut(lease_id)
            .ok_or_else(|| "lease not found".to_string())?;
        lease.release();
        append_provider_log(&mut self.logs, 2, LogKind::LeaseEvent, lease_id);
        Ok(())
    }

    fn deploy_package(
        &mut self,
        lease_id: &str,
        package: ProviderPackage,
    ) -> Result<ProviderDeployment, String> {
        let layout = HotPocketStateLayout::canonical("state");
        let plan = EverNodeDeploymentPlan::canonical();
        plan.validate_inputs(
            &package.package_hash,
            &package.runtime_version,
            &package.rustrig_hashes,
            &layout,
        )?;
        let lease = self
            .leases
            .get_mut(lease_id)
            .ok_or_else(|| "lease not found".to_string())?;
        let runtime_id = format!("runtime-{lease_id}");
        let config = HotPocketRuntimeConfig::arena_vanguard("state");
        let mut runtime = RuntimeLifecycle::new(&runtime_id, config);
        runtime.start();
        lease.mark_running();
        lease.audit.push(runtime_id.clone());
        self.runtimes.insert(runtime_id.clone(), runtime);
        self.metrics.record_deployment();
        self.metrics.collect_runtime(60.0, 3.0);
        self.metrics.snapshot.federation.node_count = 1;
        append_provider_log(&mut self.logs, 3, LogKind::Deployment, &runtime_id);
        Ok(ProviderDeployment {
            deployment_id: format!("deployment-{lease_id}"),
            lease_id: lease_id.into(),
            runtime_id,
            steps: plan.step_names(),
        })
    }

    fn start_runtime(&mut self, runtime_id: &str) -> Result<ProviderRuntimeHandle, String> {
        let runtime = self
            .runtimes
            .get_mut(runtime_id)
            .ok_or_else(|| "runtime not found".to_string())?;
        runtime.start();
        append_provider_log(&mut self.logs, 4, LogKind::Runtime, runtime_id);
        self.runtime_handle(runtime_id)
    }

    fn stop_runtime(&mut self, runtime_id: &str) -> Result<ProviderRuntimeHandle, String> {
        let runtime = self
            .runtimes
            .get_mut(runtime_id)
            .ok_or_else(|| "runtime not found".to_string())?;
        runtime.stop();
        append_provider_log(&mut self.logs, 5, LogKind::Runtime, runtime_id);
        self.runtime_handle(runtime_id)
    }

    fn restart_runtime(&mut self, runtime_id: &str) -> Result<ProviderRuntimeHandle, String> {
        let runtime = self
            .runtimes
            .get_mut(runtime_id)
            .ok_or_else(|| "runtime not found".to_string())?;
        runtime.restart();
        append_provider_log(&mut self.logs, 6, LogKind::Runtime, runtime_id);
        self.runtime_handle(runtime_id)
    }

    fn collect_health(&self, runtime_id: &str) -> Result<RuntimeHealth, String> {
        let runtime = self
            .runtimes
            .get(runtime_id)
            .ok_or_else(|| "runtime not found".to_string())?;
        Ok(collect_runtime_health(runtime_id, &runtime.process_state))
    }

    fn collect_metrics(&self) -> Result<MetricsSnapshot, String> {
        Ok(self.metrics.snapshot.clone())
    }

    fn perform_recovery(&mut self, runtime_id: &str) -> Result<ProviderRecoveryReport, String> {
        if !self.runtimes.contains_key(runtime_id) {
            return Err("runtime not found".into());
        }
        let report = recover_runtime(runtime_id, RecoveryRoots::sample())?;
        self.metrics.snapshot.federation.recovery_duration_ms = 1;
        append_provider_log(&mut self.logs, 7, LogKind::Recovery, runtime_id);
        for lease in self.leases.values_mut() {
            if lease.audit.iter().any(|e| e == runtime_id) {
                lease.state = EverNodeLeaseState::Running;
            }
        }
        Ok(report)
    }
}
