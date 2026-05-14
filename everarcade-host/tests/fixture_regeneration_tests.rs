use everarcade_host::fixture::generate_fixture_bytes;

#[test]
fn regenerated_fixture_matches_committed_fixture() {
    let generated = generate_fixture_bytes().unwrap();
    let committed = std::fs::read(format!(
        "{}/tests/fixtures/civilization_package.bin",
        env!("CARGO_MANIFEST_DIR")
    ))
    .unwrap();
    assert_eq!(generated, committed);
}
