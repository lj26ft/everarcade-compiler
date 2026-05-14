use serde::{Deserialize, Serialize};

use crate::civilization::CivilizationPackage;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalPackageEnvelope {
    pub version: u64,
    pub package_root: [u8; 32],
    pub payload_root: [u8; 32],
    pub replay_root: [u8; 32],
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalPackage {
    pub envelope: CanonicalPackageEnvelope,
    pub package: CivilizationPackage,
}
