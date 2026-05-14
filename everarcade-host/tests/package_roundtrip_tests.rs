use everarcade_host::fixture::generate_fixture_bytes;
use execution_core::codec::{package_decode::decode_package, package_encode::encode_package};

#[test]
fn host_package_roundtrip_stable() {
    let bytes = generate_fixture_bytes().unwrap();
    let package = decode_package(&bytes).unwrap();
    let reencoded = encode_package(&package);
    assert_eq!(bytes, reencoded);
}
