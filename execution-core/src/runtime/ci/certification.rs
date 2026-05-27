use super::{
    CertifiedArtifactIntegrity, SovereignGovernanceRuntime, SovereignReleaseLineageRuntime,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SovereignReleaseCertificationRuntime;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SovereignReleaseCertificate {
    pub release_id: String,
    pub lineage_proof: String,
    pub artifact_hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SovereignReleaseCertificationResult {
    pub certified: bool,
    pub certificate: Option<SovereignReleaseCertificate>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SovereignReleaseCertificationFailure {
    pub reason: String,
}

impl SovereignReleaseCertificationRuntime {
    pub fn certify(
        &self,
        release_id: &str,
        validation_closed: bool,
        replay_equivalent: bool,
        reproducible: bool,
        lineage: &SovereignReleaseLineageRuntime,
        integrity: &CertifiedArtifactIntegrity,
        governance: &SovereignGovernanceRuntime,
    ) -> Result<SovereignReleaseCertificationResult, SovereignReleaseCertificationFailure> {
        if !(validation_closed && replay_equivalent && reproducible) {
            return Err(SovereignReleaseCertificationFailure {
                reason: "validation closure incomplete".into(),
            });
        }
        if !lineage.verify_continuity() {
            return Err(SovereignReleaseCertificationFailure {
                reason: "lineage continuity violated".into(),
            });
        }
        if !integrity.integrity_ok {
            return Err(SovereignReleaseCertificationFailure {
                reason: "artifact integrity failure".into(),
            });
        }
        governance
            .enforce_certified_state(true)
            .map_err(|e| SovereignReleaseCertificationFailure { reason: e })?;
        Ok(SovereignReleaseCertificationResult {
            certified: true,
            certificate: Some(SovereignReleaseCertificate {
                release_id: release_id.to_string(),
                lineage_proof: lineage.proof_id.clone(),
                artifact_hash: integrity.artifact_hash.clone(),
            }),
        })
    }
}
