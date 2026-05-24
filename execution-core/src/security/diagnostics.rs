use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecurityDiagnosticsEnvelope {
    pub fault_detected: bool,
    pub fault_type: String,
    pub recovery_possible: bool,
    pub quarantined: bool,
    pub corruption_offset: u64,
    pub replay_safe: bool,
    pub deterministic_fault: bool,
}

impl SecurityDiagnosticsEnvelope {
    pub fn clean() -> Self {
        Self {
            fault_detected: false,
            fault_type: "none".to_string(),
            recovery_possible: true,
            quarantined: false,
            corruption_offset: 0,
            replay_safe: true,
            deterministic_fault: true,
        }
    }

    pub fn fault(
        fault_type: &str,
        offset: u64,
        recovery_possible: bool,
        quarantined: bool,
    ) -> Self {
        Self {
            fault_detected: true,
            fault_type: fault_type.to_string(),
            recovery_possible,
            quarantined,
            corruption_offset: offset,
            replay_safe: !quarantined,
            deterministic_fault: true,
        }
    }
}
