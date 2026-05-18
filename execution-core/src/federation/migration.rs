use crate::federation::exchange::{ContinuityExportProof, ContinuityImportReport};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MigrationDescriptor {
    pub world_id: String,
    pub source_operator: String,
    pub destination_operator: String,
    pub freeze_height: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MigrationProof {
    pub continuity_export_proof: ContinuityExportProof,
    pub destination_import_report: ContinuityImportReport,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MigrationContinuityReport {
    pub migration_descriptor: MigrationDescriptor,
    pub source_export_verified: bool,
    pub destination_import_verified: bool,
}

impl MigrationContinuityReport {
    pub fn continuity_preserved(&self) -> bool {
        self.source_export_verified && self.destination_import_verified
    }
}
