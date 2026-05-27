#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CiExecutionSummary {
    pub stages: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CiValidationSummary {
    pub replay_equivalent: bool,
    pub warnings: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CiReleaseSummary {
    pub verified: bool,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CiRuntimeReport {
    pub execution: CiExecutionSummary,
    pub validation: CiValidationSummary,
    pub release: CiReleaseSummary,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuntimeCertificationReport {
    pub summary: RuntimeCertificationSummary,
    pub evidence: RuntimeCertificationEvidence,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuntimeCertificationSummary {
    pub deterministic: bool,
    pub lineage_ok: bool,
    pub replay_equivalent: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuntimeCertificationEvidence {
    pub warning_gate_ok: bool,
    pub security_gate_ok: bool,
    pub proof_summary: String,
}
