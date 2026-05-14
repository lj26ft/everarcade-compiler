use serde::de::DeserializeOwned;

use crate::{
    civilization::CivilizationPackage,
    codec::{
        canonical_package::CanonicalPackage, package_validation::validate_envelope,
        package_version::PackageVersion,
    },
};

#[derive(Debug)]
pub enum CodecError {
    Io(bincode::Error),
    UnsupportedVersion(u64),
    TrailingBytes,
    InvalidPackageRoot,
    InvalidPayloadRoot,
    InvalidReplayRoot,
}

impl From<bincode::Error> for CodecError {
    fn from(value: bincode::Error) -> Self { Self::Io(value) }
}

pub fn decode_fully<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, CodecError> {
    let mut c = std::io::Cursor::new(bytes);
    let value: T = bincode::deserialize_from(&mut c)?;
    if c.position() != bytes.len() as u64 { return Err(CodecError::TrailingBytes); }
    Ok(value)
}

pub fn decode_package(bytes: &[u8]) -> Result<CivilizationPackage, CodecError> {
    let canonical: CanonicalPackage = decode_fully(bytes)?;
    PackageVersion::decode(canonical.envelope.version as u32)?;
    validate_envelope(&canonical.package, &canonical.envelope)?;
    Ok(canonical.package)
}
