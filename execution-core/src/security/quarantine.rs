use super::diagnostics::SecurityDiagnosticsEnvelope;

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum QuarantineReason {
    MaliciousExecutionArtifact,
    InvalidSnapshot,
    CorruptedWitnessBundle,
    InvalidReplayWindow,
    MalformedRestorationChain,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct QuarantineReceipt {
    pub execution_id: String,
    pub reason: QuarantineReason,
    pub deterministic: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RuntimeQuarantine {
    pub receipts: Vec<QuarantineReceipt>,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct IsolationBoundary {
    pub node_id: String,
}
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RecoveryManifest {
    pub checkpoint_id: String,
    pub replay_root: String,
}
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum RecoveryEligibility {
    Eligible,
    Ineligible(String),
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct QuarantineEnvelope {
    pub quarantined: bool,
    pub boundary: IsolationBoundary,
    pub manifest: Option<RecoveryManifest>,
    pub eligibility: RecoveryEligibility,
    pub diagnostics: SecurityDiagnosticsEnvelope,
    pub receipt: Option<QuarantineReceipt>,
}
