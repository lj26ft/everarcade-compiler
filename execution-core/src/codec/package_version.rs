use crate::codec::package_decode::CodecError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PackageVersion {
    pub major: u16,
    pub minor: u16,
}

impl PackageVersion {
    pub const V1_0: Self = Self { major: 1, minor: 0 };

    pub fn encode(self) -> u32 {
        ((self.major as u32) << 16) | (self.minor as u32)
    }

    pub fn decode(raw: u32) -> Result<Self, CodecError> {
        let version = Self {
            major: (raw >> 16) as u16,
            minor: (raw & 0xffff) as u16,
        };
        if version == Self::V1_0 {
            Ok(version)
        } else {
            Err(CodecError::UnsupportedVersion(raw as u64))
        }
    }
}
