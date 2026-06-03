use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuntimeState {
    Booting,
    ValidatingPackage,
    LoadingState,
    ReplayingJournal,
    Running,
    Checkpointing,
    Recovering,
    Stopping,
    Stopped,
    Failed,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LifecycleTransition {
    pub sequence: u64,
    pub from: RuntimeState,
    pub to: RuntimeState,
    pub reason: String,
    pub timestamp_ms: u128,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RuntimeLifecycle {
    pub state: RuntimeState,
    pub transitions: Vec<LifecycleTransition>,
}

impl RuntimeLifecycle {
    pub fn boot() -> Self {
        Self {
            state: RuntimeState::Booting,
            transitions: Vec::new(),
        }
    }

    pub fn transition(&mut self, to: RuntimeState, reason: impl Into<String>) {
        let sequence = self.transitions.len() as u64 + 1;
        let from = self.state.clone();
        self.transitions.push(LifecycleTransition {
            sequence,
            from,
            to: to.clone(),
            reason: reason.into(),
            timestamp_ms: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis(),
        });
        self.state = to;
    }
}
