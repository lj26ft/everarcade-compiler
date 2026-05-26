use std::collections::BTreeSet;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ValidationCheckpoint {
    pub completed_stages: BTreeSet<String>,
}

#[derive(Clone, Debug, Default)]
pub struct ValidationCheckpointRuntime {
    checkpoint: ValidationCheckpoint,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ValidationResumeState {
    pub resume_from: BTreeSet<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidationRecoveryWindow {
    pub start: u64,
    pub end: u64,
}

impl ValidationCheckpointRuntime {
    pub fn mark_completed(&mut self, stage: String) {
        self.checkpoint.completed_stages.insert(stage);
    }
    pub fn checkpoint(&self) -> ValidationCheckpoint {
        self.checkpoint.clone()
    }
    pub fn resume_state(&self) -> ValidationResumeState {
        ValidationResumeState {
            resume_from: self.checkpoint.completed_stages.clone(),
        }
    }
}
