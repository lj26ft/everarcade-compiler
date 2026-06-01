use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RuntimeMetrics {
    pub ticks_per_sec: f64,
    pub tick_latency_ms: f64,
    pub replay_throughput_bytes: u64,
    pub checkpoint_throughput_bytes: u64,
}
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FederationMetrics {
    pub node_count: usize,
    pub convergence_latency_ms: u64,
    pub checkpoint_sync_latency_ms: u64,
    pub recovery_duration_ms: u64,
    pub partition_events: u64,
}
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DeploymentMetrics {
    pub deployment_count: u64,
    pub upgrade_count: u64,
    pub rollback_count: u64,
    pub failed_deployments: u64,
}
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PlayerMetrics {
    pub active_sessions: u64,
    pub connected_players: u64,
    pub world_count: u64,
    pub civilization_count: u64,
}
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MetricsSnapshot {
    pub runtime: RuntimeMetrics,
    pub federation: FederationMetrics,
    pub deployment: DeploymentMetrics,
    pub players: PlayerMetrics,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct MetricsCollector {
    pub snapshot: MetricsSnapshot,
}
impl MetricsCollector {
    pub fn collect_runtime(&mut self, ticks_per_sec: f64, tick_latency_ms: f64) {
        self.snapshot.runtime.ticks_per_sec = ticks_per_sec;
        self.snapshot.runtime.tick_latency_ms = tick_latency_ms;
    }
    pub fn record_deployment(&mut self) {
        self.snapshot.deployment.deployment_count += 1;
    }
    pub fn record_upgrade(&mut self) {
        self.snapshot.deployment.upgrade_count += 1;
    }
    pub fn record_rollback(&mut self) {
        self.snapshot.deployment.rollback_count += 1;
    }
    pub fn record_failure(&mut self) {
        self.snapshot.deployment.failed_deployments += 1;
    }
    pub fn set_players(&mut self, sessions: u64, players: u64, worlds: u64, civilizations: u64) {
        self.snapshot.players = PlayerMetrics {
            active_sessions: sessions,
            connected_players: players,
            world_count: worlds,
            civilization_count: civilizations,
        };
    }
}
