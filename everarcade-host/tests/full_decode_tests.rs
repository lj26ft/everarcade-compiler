use everarcade_host::fixture::generate_fixture_bytes;
use execution_core::codec::package_decode::decode_fully;

#[test]
fn decode_fully_rejects_trailing_garbage() {
    let mut bytes = generate_fixture_bytes().unwrap();
    bytes.extend_from_slice(&[9]);
    let result = decode_fully::<execution_core::codec::canonical_package::CanonicalPackage>(&bytes);
    assert!(result.is_err());
}
