#[test]
fn property_stub_manifest_exists() {
    assert_eq!(
        everarcade_rustrig_market::certified_status().contains("PASS"),
        true
    );
}
