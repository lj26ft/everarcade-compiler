use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct LeaseId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LeaseStatus {
    Pending,
    Allocated,
    Starting,
    Running,
    Recovering,
    Draining,
    Stopped,
    Failed,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LeaseResources {
    pub cpu_cores: u32,
    pub memory_mb: u64,
    pub storage_gb: u64,
    pub bandwidth_mbps: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LeaseCapacity {
    pub max_runtimes: usize,
    pub resources: LeaseResources,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LeasePolicy {
    pub allow_failover: bool,
    pub allow_recovery: bool,
    pub max_runtime_moves: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LeaseRuntime {
    pub runtime_id: String,
    pub node_id: String,
    pub version: String,
    pub status: LeaseStatus,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LeaseAssignment {
    pub lease_id: LeaseId,
    pub runtime_id: String,
    pub node_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LeaseAllocation {
    pub id: LeaseId,
    pub game_id: String,
    pub status: LeaseStatus,
    pub resources: LeaseResources,
    pub runtime: Option<LeaseRuntime>,
    pub move_count: u32,
    pub audit_events: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LeaseManager {
    pub capacity: LeaseCapacity,
    pub policy: LeasePolicy,
    pub leases: BTreeMap<LeaseId, LeaseAllocation>,
    next_id: u64,
}

impl LeaseManager {
    pub fn new(capacity: LeaseCapacity, policy: LeasePolicy) -> Self {
        Self {
            capacity,
            policy,
            leases: BTreeMap::new(),
            next_id: 1,
        }
    }

    pub fn arena_vanguard() -> Self {
        Self::new(
            LeaseCapacity {
                max_runtimes: 100,
                resources: LeaseResources {
                    cpu_cores: 400,
                    memory_mb: 819_200,
                    storage_gb: 20_000,
                    bandwidth_mbps: 100_000,
                },
            },
            LeasePolicy {
                allow_failover: true,
                allow_recovery: true,
                max_runtime_moves: 8,
            },
        )
    }

    pub fn allocate_lease(
        &mut self,
        game_id: impl Into<String>,
        resources: LeaseResources,
    ) -> Result<LeaseAllocation, String> {
        if self.leases.len() >= self.capacity.max_runtimes {
            return Err("lease exhaustion".into());
        }
        let id = LeaseId(format!("lease-{:06}", self.next_id));
        self.next_id += 1;
        let mut allocation = LeaseAllocation {
            id: id.clone(),
            game_id: game_id.into(),
            status: LeaseStatus::Allocated,
            resources,
            runtime: None,
            move_count: 0,
            audit_events: vec!["allocate lease".into()],
        };
        allocation.audit_events.push("status=Allocated".into());
        self.leases.insert(id, allocation.clone());
        Ok(allocation)
    }

    pub fn release_lease(&mut self, lease_id: &LeaseId) -> Result<(), String> {
        let lease = self
            .leases
            .get_mut(lease_id)
            .ok_or_else(|| "lease not found".to_string())?;
        lease.status = LeaseStatus::Stopped;
        lease.runtime = None;
        lease.audit_events.push("release lease".into());
        Ok(())
    }

    pub fn assign_runtime(
        &mut self,
        lease_id: &LeaseId,
        runtime_id: impl Into<String>,
        node_id: impl Into<String>,
        version: impl Into<String>,
    ) -> Result<LeaseAssignment, String> {
        let lease = self
            .leases
            .get_mut(lease_id)
            .ok_or_else(|| "lease not found".to_string())?;
        let runtime_id = runtime_id.into();
        let node_id = node_id.into();
        lease.status = LeaseStatus::Running;
        lease.runtime = Some(LeaseRuntime {
            runtime_id: runtime_id.clone(),
            node_id: node_id.clone(),
            version: version.into(),
            status: LeaseStatus::Running,
        });
        lease.audit_events.push("runtime assigned".into());
        Ok(LeaseAssignment {
            lease_id: lease_id.clone(),
            runtime_id,
            node_id,
        })
    }

    pub fn move_runtime(
        &mut self,
        lease_id: &LeaseId,
        target_node: impl Into<String>,
    ) -> Result<LeaseAssignment, String> {
        let lease = self
            .leases
            .get_mut(lease_id)
            .ok_or_else(|| "lease not found".to_string())?;
        if lease.move_count >= self.policy.max_runtime_moves {
            return Err("move limit reached".into());
        }
        let runtime = lease
            .runtime
            .as_mut()
            .ok_or_else(|| "runtime not assigned".to_string())?;
        runtime.node_id = target_node.into();
        lease.move_count += 1;
        lease.audit_events.push("move runtime".into());
        Ok(LeaseAssignment {
            lease_id: lease_id.clone(),
            runtime_id: runtime.runtime_id.clone(),
            node_id: runtime.node_id.clone(),
        })
    }

    pub fn failover_runtime(
        &mut self,
        lease_id: &LeaseId,
        target_node: impl Into<String>,
    ) -> Result<LeaseAssignment, String> {
        if !self.policy.allow_failover {
            return Err("failover disabled".into());
        }
        let assignment = self.move_runtime(lease_id, target_node)?;
        if let Some(lease) = self.leases.get_mut(lease_id) {
            lease.audit_events.push("failover runtime".into());
        }
        Ok(assignment)
    }

    pub fn recover_runtime(&mut self, lease_id: &LeaseId) -> Result<(), String> {
        if !self.policy.allow_recovery {
            return Err("recovery disabled".into());
        }
        let lease = self
            .leases
            .get_mut(lease_id)
            .ok_or_else(|| "lease not found".to_string())?;
        lease.status = LeaseStatus::Recovering;
        if let Some(runtime) = lease.runtime.as_mut() {
            runtime.status = LeaseStatus::Recovering;
        }
        lease.audit_events.push("recover runtime".into());
        lease.status = LeaseStatus::Running;
        if let Some(runtime) = lease.runtime.as_mut() {
            runtime.status = LeaseStatus::Running;
        }
        Ok(())
    }

    pub fn audit_lease_usage(&self) -> Vec<String> {
        self.leases
            .values()
            .flat_map(|l| {
                l.audit_events
                    .iter()
                    .map(move |e| format!("{}:{}", l.id.0, e))
            })
            .collect()
    }
}
