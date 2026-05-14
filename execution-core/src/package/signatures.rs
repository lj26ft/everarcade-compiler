use crate::hashing;

use super::bundle::ExecutionPackage;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PackageSignature {
    pub signer: String,
    pub signature: String,
}

pub fn sign_placeholder(package: &ExecutionPackage, signer: &str) -> PackageSignature {
    let signature =
        hashing::hash_bytes(format!("{}:{}", signer, package.manifest.package_hash).as_bytes());
    PackageSignature {
        signer: signer.to_string(),
        signature,
    }
}
