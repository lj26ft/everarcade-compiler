use crate::health::{HealthState, RuntimeHealth};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertLevel {
    Info,
    Warning,
    Critical,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertTrigger {
    RuntimeStalled,
    CheckpointOverdue,
    NodeLost,
    PartitionDetected,
    DeploymentFailure,
    RecoveryFailure,
    LeaseExhaustion,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Alert {
    pub level: AlertLevel,
    pub trigger: AlertTrigger,
    pub subject: String,
    pub message: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AlertManager {
    pub alerts: Vec<Alert>,
}
impl AlertManager {
    pub fn emit(
        &mut self,
        level: AlertLevel,
        trigger: AlertTrigger,
        subject: impl Into<String>,
        message: impl Into<String>,
    ) -> Alert {
        let alert = Alert {
            level,
            trigger,
            subject: subject.into(),
            message: message.into(),
        };
        self.alerts.push(alert.clone());
        alert
    }
    pub fn evaluate_health(&mut self, health: &RuntimeHealth) -> Vec<Alert> {
        let before = self.alerts.len();
        match health.state {
            HealthState::Healthy => {}
            HealthState::Warning => {
                self.emit(
                    AlertLevel::Warning,
                    AlertTrigger::RuntimeStalled,
                    health.runtime_id.clone(),
                    "runtime warning",
                );
            }
            HealthState::Critical => {
                self.emit(
                    AlertLevel::Critical,
                    AlertTrigger::CheckpointOverdue,
                    health.runtime_id.clone(),
                    "checkpoint overdue or resources critical",
                );
            }
            HealthState::Failed => {
                self.emit(
                    AlertLevel::Critical,
                    AlertTrigger::NodeLost,
                    health.runtime_id.clone(),
                    "runtime failed",
                );
            }
        }
        self.alerts[before..].to_vec()
    }
}
