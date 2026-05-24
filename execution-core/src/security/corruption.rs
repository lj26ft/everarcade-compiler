use super::diagnostics::SecurityDiagnosticsEnvelope;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenericSecurityEnvelope {
    pub accepted: bool,
    pub label: &'static str,
    pub diagnostics: SecurityDiagnosticsEnvelope,
}

pub fn deterministic_reject(
    label: &'static str,
    offset: u64,
    recovery_possible: bool,
) -> GenericSecurityEnvelope {
    GenericSecurityEnvelope {
        accepted: false,
        label,
        diagnostics: SecurityDiagnosticsEnvelope::fault(
            label,
            offset,
            recovery_possible,
            !recovery_possible,
        ),
    }
}
