use crate::{
    civilization::CivilizationPackage,
    codec::{
        package_decode::{decode_package, CodecError},
        package_encode::encode_package,
    },
};

pub fn assert_roundtrip(package: &CivilizationPackage) -> Result<Vec<u8>, CodecError> {
    let bytes = encode_package(package);
    let decoded = decode_package(&bytes)?;
    let encoded_again = encode_package(&decoded);
    if bytes != encoded_again {
        return Err(CodecError::TrailingBytes);
    }
    Ok(bytes)
}
