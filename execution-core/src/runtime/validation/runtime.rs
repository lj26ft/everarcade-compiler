#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SovereignReleaseArtifact {
    pub id: String,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SovereignReleaseManifest {
    pub artifacts: Vec<SovereignReleaseArtifact>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SovereignReleaseVerification {
    pub reproducible: bool,
}
#[derive(Clone, Debug, Default)]
pub struct SovereignReleaseRuntime;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidationReplayRuntime;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidationReplaySession {
    pub session_id: String,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidationReplayCursor {
    pub stage_index: usize,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidationReplayRecovery {
    pub restored: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReleaseCandidateRuntime;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReleaseCandidateManifest {
    pub artifact_ids: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReleaseCandidateVerification {
    pub deterministic: bool,
    pub replay_equivalent: bool,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReleaseCandidateProof {
    pub continuity_lineage: String,
}
