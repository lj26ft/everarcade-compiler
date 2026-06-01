use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthState {
    Healthy,
    Warning,
    Critical,
    Failed,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuntimeHealth {
    pub runtime_id: String,
    pub runtime_alive: bool,
    pub tick_progress: u64,
    pub replay_growth_bytes: u64,
    pub checkpoint_age_seconds: u64,
    pub recovery_status: String,
    pub memory_usage_mb: u64,
    pub cpu_usage_percent: f64,
    pub disk_usage_percent: f64,
    pub network_status: String,
    pub state: HealthState,
}

impl RuntimeHealth {
    pub fn healthy(runtime_id: impl Into<String>, tick_progress: u64) -> Self {
        Self {
            runtime_id: runtime_id.into(),
            runtime_alive: true,
            tick_progress,
            replay_growth_bytes: 1,
            checkpoint_age_seconds: 0,
            recovery_status: "idle".into(),
            memory_usage_mb: 256,
            cpu_usage_percent: 10.0,
            disk_usage_percent: 5.0,
            network_status: "connected".into(),
            state: HealthState::Healthy,
        }
    }

    pub fn evaluate(&mut self) -> HealthState {
        self.state = if !self.runtime_alive {
            HealthState::Failed
        } else if self.checkpoint_age_seconds > 300 || self.cpu_usage_percent > 95.0 {
            HealthState::Critical
        } else if self.replay_growth_bytes == 0 || self.disk_usage_percent > 85.0 {
            HealthState::Warning
        } else {
            HealthState::Healthy
        };
        self.state.clone()
    }
}
