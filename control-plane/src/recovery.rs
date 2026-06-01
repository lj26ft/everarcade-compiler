use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryPlan {
    pub runtime_id: String,
    pub steps: Vec<String>,
    pub automated: bool,
}
pub fn automated_recovery_plan(runtime_id: impl Into<String>) -> RecoveryPlan {
    RecoveryPlan {
        runtime_id: runtime_id.into(),
        automated: true,
        steps: vec![
            "checkpoint restore".into(),
            "replay recovery".into(),
            "node rejoin".into(),
        ],
    }
}
