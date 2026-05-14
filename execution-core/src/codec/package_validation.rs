use sha2::{Digest, Sha256};

use crate::{
    civilization::CivilizationPackage,
    codec::{canonical_package::CanonicalPackageEnvelope, package_decode::CodecError},
};

pub fn package_root(package: &CivilizationPackage) -> [u8; 32] {
    let bytes = bincode::serialize(package).expect("serialize package");
    let digest = Sha256::digest(bytes);
    let mut out = [0u8; 32];
    out.copy_from_slice(&digest);
    out
}

pub fn validate_envelope(
    package: &CivilizationPackage,
    envelope: &CanonicalPackageEnvelope,
) -> Result<(), CodecError> {
    if envelope.package_root != package_root(package) {
        return Err(CodecError::InvalidPackageRoot);
    }
    if envelope.payload_root != package.proof_root {
        return Err(CodecError::InvalidPayloadRoot);
    }
    if envelope.replay_root != package.replay_root {
        return Err(CodecError::InvalidReplayRoot);
    }
    Ok(())
}
