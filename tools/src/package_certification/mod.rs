use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CertificationStage {
    Build,
    Hash,
    Validate,
    ReplayCheck,
    AbiCheck,
    Sign,
    Publish,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationReport {
    pub package_id: String,
    pub stages: Vec<CertificationStage>,
    pub package_hash: String,
    pub signature: String,
    pub studio_visible: bool,
}

pub fn certification_stages() -> Vec<CertificationStage> {
    vec![
        CertificationStage::Build,
        CertificationStage::Hash,
        CertificationStage::Validate,
        CertificationStage::ReplayCheck,
        CertificationStage::AbiCheck,
        CertificationStage::Sign,
        CertificationStage::Publish,
    ]
}

pub fn certify_package(
    package_id: &str,
    bytes: &[u8],
    validation_passed: bool,
) -> CertificationReport {
    let package_hash = hex::encode(Sha256::digest(bytes));
    let signature = format!("signed:{package_hash}");
    CertificationReport {
        package_id: package_id.to_owned(),
        stages: certification_stages(),
        package_hash,
        signature,
        studio_visible: validation_passed,
    }
}

pub fn appears_in_studio(report: &CertificationReport) -> bool {
    report.studio_visible
        && report.stages == certification_stages()
        && report.signature.starts_with("signed:")
}
