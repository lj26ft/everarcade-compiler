use everarcade_host::fixture::generate_fixture_bytes;
use execution_core::codec::package_decode::decode_package;

#[test]
fn fixture_roundtrip_decode_works() {
    let bytes = generate_fixture_bytes().unwrap();
    decode_package(&bytes).unwrap();
}
