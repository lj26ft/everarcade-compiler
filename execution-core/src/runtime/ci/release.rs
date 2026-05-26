#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SovereignReleaseCandidate {
    pub id: String,
    pub artifact_hash: String,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SovereignReleaseExecution {
    pub candidate_id: String,
    pub reproducible: bool,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SovereignReleaseVerification {
    pub lineage_ok: bool,
    pub signatures_ok: bool,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SovereignReleaseManifest {
    pub ancestry: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SovereignReleaseLineage {
    pub root: String,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SovereignReleaseContinuity {
    pub append_only: bool,
}
#[derive(Clone, Debug, Default)]
pub struct SovereignReleaseAutomationRuntime;
impl SovereignReleaseAutomationRuntime {
    pub fn generate(&self, id: &str) -> SovereignReleaseCandidate {
        SovereignReleaseCandidate {
            id: id.to_string(),
            artifact_hash: format!("sha256:{}", id),
        }
    }
}
