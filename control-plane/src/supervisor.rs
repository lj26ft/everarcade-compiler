use crate::health::RuntimeHealth;
use crate::leases::{LeaseId, LeaseManager, LeaseStatus};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeProcess {
    pub runtime_id: String,
    pub lease_id: LeaseId,
    pub version: String,
    pub status: LeaseStatus,
    pub recovery_events: Vec<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RuntimeSupervisor {
    pub runtimes: BTreeMap<String, RuntimeProcess>,
}
impl RuntimeSupervisor {
    pub fn start_runtime(
        &mut self,
        leases: &mut LeaseManager,
        lease_id: &LeaseId,
        runtime_id: impl Into<String>,
        node_id: impl Into<String>,
        version: impl Into<String>,
    ) -> Result<RuntimeProcess, String> {
        let runtime_id = runtime_id.into();
        let version = version.into();
        leases.assign_runtime(lease_id, runtime_id.clone(), node_id, version.clone())?;
        let process = RuntimeProcess {
            runtime_id: runtime_id.clone(),
            lease_id: lease_id.clone(),
            version,
            status: LeaseStatus::Running,
            recovery_events: vec!["start runtime".into()],
        };
        self.runtimes.insert(runtime_id, process.clone());
        Ok(process)
    }
    pub fn stop_runtime(&mut self, runtime_id: &str) -> Result<(), String> {
        let runtime = self
            .runtimes
            .get_mut(runtime_id)
            .ok_or_else(|| "runtime not found".to_string())?;
        runtime.status = LeaseStatus::Stopped;
        runtime.recovery_events.push("stop runtime".into());
        Ok(())
    }
    pub fn restart_runtime(&mut self, runtime_id: &str) -> Result<(), String> {
        let runtime = self
            .runtimes
            .get_mut(runtime_id)
            .ok_or_else(|| "runtime not found".to_string())?;
        runtime.status = LeaseStatus::Running;
        runtime.recovery_events.push("restart runtime".into());
        Ok(())
    }
    pub fn upgrade_runtime(
        &mut self,
        runtime_id: &str,
        version: impl Into<String>,
    ) -> Result<(), String> {
        let runtime = self
            .runtimes
            .get_mut(runtime_id)
            .ok_or_else(|| "runtime not found".to_string())?;
        runtime.version = version.into();
        runtime.recovery_events.push("upgrade runtime".into());
        Ok(())
    }
    pub fn recover_runtime(
        &mut self,
        leases: &mut LeaseManager,
        runtime_id: &str,
    ) -> Result<(), String> {
        let runtime = self
            .runtimes
            .get_mut(runtime_id)
            .ok_or_else(|| "runtime not found".to_string())?;
        runtime.status = LeaseStatus::Recovering;
        runtime.recovery_events.extend([
            "checkpoint restore".into(),
            "replay recovery".into(),
            "node rejoin".into(),
        ]);
        leases.recover_runtime(&runtime.lease_id)?;
        runtime.status = LeaseStatus::Running;
        Ok(())
    }
    pub fn verify_runtime(&self, runtime_id: &str) -> bool {
        self.runtimes
            .get(runtime_id)
            .map(|r| r.status == LeaseStatus::Running)
            .unwrap_or(false)
    }
    pub fn health(&self, runtime_id: &str) -> Option<RuntimeHealth> {
        self.runtimes
            .get(runtime_id)
            .map(|_| RuntimeHealth::healthy(runtime_id, 1))
    }
}
