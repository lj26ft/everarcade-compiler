use super::diagnostics::SecurityDiagnosticsEnvelope;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReplayCorruptionScenario {
    TruncatedReplay,
    OutOfOrderReplay,
    InvalidOperationSequence,
    DuplicateReplayEntry,
    ReceiptMismatch,
    ReplayRootMismatch,
    CheckpointLineageBreak,
    InvalidEntityLineage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayValidationResult {
    pub accepted: bool,
    pub failure_location: Option<usize>,
    pub restoration_possible: bool,
    pub diagnostics: SecurityDiagnosticsEnvelope,
}

pub fn validate_replay_scenario(s: ReplayCorruptionScenario) -> ReplayValidationResult {
    let (fault, offset, restore) = match s {
        ReplayCorruptionScenario::TruncatedReplay => ("truncated_replay", 4, true),
        ReplayCorruptionScenario::OutOfOrderReplay => ("out_of_order_replay", 9, true),
        ReplayCorruptionScenario::InvalidOperationSequence => {
            ("invalid_operation_sequence", 12, false)
        }
        ReplayCorruptionScenario::DuplicateReplayEntry => ("duplicate_replay_entry", 7, true),
        ReplayCorruptionScenario::ReceiptMismatch => ("receipt_mismatch", 18, false),
        ReplayCorruptionScenario::ReplayRootMismatch => ("replay_root_mismatch", 21, false),
        ReplayCorruptionScenario::CheckpointLineageBreak => ("checkpoint_lineage_break", 30, false),
        ReplayCorruptionScenario::InvalidEntityLineage => ("invalid_entity_lineage", 27, false),
    };
    ReplayValidationResult {
        accepted: false,
        failure_location: Some(offset as usize),
        restoration_possible: restore,
        diagnostics: SecurityDiagnosticsEnvelope::fault(fault, offset, restore, !restore),
    }
}
