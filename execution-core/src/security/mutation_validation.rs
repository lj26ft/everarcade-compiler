use super::diagnostics::SecurityDiagnosticsEnvelope;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidMutationRecord {
    pub mutation_id: String,
    pub reason: &'static str,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContinuityViolation {
    pub lineage: String,
    pub reason: &'static str,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MutationValidationEnvelope {
    pub accepted: bool,
    pub record: InvalidMutationRecord,
    pub continuity: Option<ContinuityViolation>,
    pub diagnostics: SecurityDiagnosticsEnvelope,
}

pub fn reject_hostile_mutation(reason: &'static str) -> MutationValidationEnvelope {
    MutationValidationEnvelope {
        accepted: false,
        record: InvalidMutationRecord {
            mutation_id: "deterministic".into(),
            reason,
        },
        continuity: Some(ContinuityViolation {
            lineage: "entity-lineage".into(),
            reason,
        }),
        diagnostics: SecurityDiagnosticsEnvelope::fault(reason, 0, false, true),
    }
}
