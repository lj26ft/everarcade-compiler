use everarcade_host::fixture::generate_fixture_bytes;
use execution_core::codec::package_decode::decode_package;

#[test]
fn fixture_corruption_is_rejected() {
    let mut bytes = generate_fixture_bytes().unwrap();
    bytes.push(42);
    assert!(decode_package(&bytes).is_err());
}
