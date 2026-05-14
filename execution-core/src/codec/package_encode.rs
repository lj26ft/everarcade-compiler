use crate::{
    civilization::CivilizationPackage,
    codec::{
        canonical_package::{CanonicalPackage, CanonicalPackageEnvelope},
        package_validation::package_root,
        package_version::PackageVersion,
    },
};

pub fn encode_package(package: &CivilizationPackage) -> Vec<u8> {
    let envelope = CanonicalPackageEnvelope {
        version: PackageVersion::V1_0.encode() as u64,
        package_root: package_root(package),
        payload_root: package.proof_root,
        replay_root: package.replay_root,
    };
    let canonical = CanonicalPackage { envelope, package: package.clone() };
    bincode::serialize(&canonical).expect("canonical package encode")
}
