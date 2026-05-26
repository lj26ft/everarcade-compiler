use super::{ValidationCheckpoint, ValidationResumeState};

#[derive(Clone, Debug, Default)]
pub struct ValidationRecoveryRuntime;

impl ValidationRecoveryRuntime {
    pub fn recover(&self, checkpoint: &ValidationCheckpoint) -> ValidationResumeState {
        ValidationResumeState {
            resume_from: checkpoint.completed_stages.clone(),
        }
    }
}
