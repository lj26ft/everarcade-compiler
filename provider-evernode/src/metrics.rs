use control_plane::metrics::MetricsSnapshot;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ProviderMetricExports {
    pub runtime_metrics: u64,
    pub lease_metrics: u64,
    pub deployment_metrics: u64,
    pub host_metrics: u64,
    pub recovery_metrics: u64,
    pub federation_metrics: u64,
}

impl ProviderMetricExports {
    pub fn from_snapshot(
        snapshot: &MetricsSnapshot,
        leases: usize,
        hosts: usize,
        federation_nodes: usize,
    ) -> Self {
        Self {
            runtime_metrics: snapshot.runtime.ticks_per_sec as u64,
            lease_metrics: leases as u64,
            deployment_metrics: snapshot.deployment.deployment_count,
            host_metrics: hosts as u64,
            recovery_metrics: snapshot.federation.recovery_duration_ms,
            federation_metrics: federation_nodes as u64,
        }
    }
}
