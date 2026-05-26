use super::{
    branch::ReplayForkVerification,
    provenance::{ReplayProvenanceProof, ReplayProvenanceRoot},
};
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoricalReplayCorruptionReport {
    pub valid: bool,
    pub reasons: Vec<String>,
}
pub fn detect_corruption(
    branch: &ReplayForkVerification,
    proof: &ReplayProvenanceProof,
    expected: &ReplayProvenanceRoot,
) -> HistoricalReplayCorruptionReport {
    let mut reasons = Vec::new();
    if !branch.valid {
        reasons.push("replay_branch_forgery".into());
    }
    if !proof.verify(expected) {
        reasons.push("replay_provenance_corruption".into());
    }
    HistoricalReplayCorruptionReport {
        valid: reasons.is_empty(),
        reasons,
    }
}
