use super::diagnostics::SecurityDiagnosticsEnvelope;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IsolationBoundary {
    pub node_id: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryManifest {
    pub checkpoint_id: String,
    pub replay_root: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecoveryEligibility {
    Eligible,
    Ineligible(&'static str),
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuarantineEnvelope {
    pub quarantined: bool,
    pub boundary: IsolationBoundary,
    pub manifest: Option<RecoveryManifest>,
    pub eligibility: RecoveryEligibility,
    pub diagnostics: SecurityDiagnosticsEnvelope,
}
