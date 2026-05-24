use super::diagnostics::SecurityDiagnosticsEnvelope;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArchiveTamperScenario {
    ArchiveTruncation,
    ArchiveMutation,
    InvalidArchiveRoot,
    CorruptedCheckpoint,
    MissingRestorationManifest,
    CrossEraArchiveMismatch,
    PartialArchiveReplay,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchiveValidationResult {
    pub accepted: bool,
    pub rejection_code: &'static str,
    pub diagnostics: SecurityDiagnosticsEnvelope,
}
pub fn validate_archive_scenario(s: ArchiveTamperScenario) -> ArchiveValidationResult {
    let (code, off, restore) = match s {
        ArchiveTamperScenario::ArchiveTruncation => ("archive_truncation", 10, true),
        ArchiveTamperScenario::ArchiveMutation => ("archive_mutation", 11, false),
        ArchiveTamperScenario::InvalidArchiveRoot => ("invalid_archive_root", 12, false),
        ArchiveTamperScenario::CorruptedCheckpoint => ("corrupted_checkpoint", 13, true),
        ArchiveTamperScenario::MissingRestorationManifest => {
            ("missing_restoration_manifest", 14, false)
        }
        ArchiveTamperScenario::CrossEraArchiveMismatch => ("cross_era_archive_mismatch", 15, false),
        ArchiveTamperScenario::PartialArchiveReplay => ("partial_archive_replay", 16, true),
    };
    ArchiveValidationResult {
        accepted: false,
        rejection_code: code,
        diagnostics: SecurityDiagnosticsEnvelope::fault(code, off, restore, !restore),
    }
}
