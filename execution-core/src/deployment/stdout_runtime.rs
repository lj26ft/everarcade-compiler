use serde::{Deserialize, Serialize};

use super::hash_hex;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeStatusEnvelope {
    pub phase: String,
    pub continuity_root: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OperationalProgressReport {
    pub operation: String,
    pub status: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeterministicRuntimeLog {
    pub lines: Vec<String>,
    pub log_hash: String,
}

pub struct StdoutRuntimeReporter;
impl StdoutRuntimeReporter {
    pub fn report(
        mut statuses: Vec<RuntimeStatusEnvelope>,
        mut progress: Vec<OperationalProgressReport>,
    ) -> DeterministicRuntimeLog {
        statuses.sort_by(|a, b| a.phase.cmp(&b.phase));
        progress.sort_by(|a, b| a.operation.cmp(&b.operation));
        let mut lines: Vec<String> = statuses
            .into_iter()
            .map(|s| format!("STATUS|{}|{}", s.phase, s.continuity_root))
            .collect();
        lines.extend(
            progress
                .into_iter()
                .map(|p| format!("PROGRESS|{}|{}", p.operation, p.status)),
        );
        let log_hash = hash_hex(bincode::serialize(&lines).expect("serializable"));
        DeterministicRuntimeLog { lines, log_hash }
    }
}
