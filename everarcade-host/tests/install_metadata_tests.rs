#[test]
fn install_priority_order_is_documented_in_script() {
    let script_path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../scripts/install.sh");
    let script = std::fs::read_to_string(script_path).unwrap();
    assert!(script.contains("PREFIX=\"${PREFIX:-$PREFIX_DEFAULT}\""));
    assert!(script.contains("--prefix"));
    assert!(script.contains("SCRIPT_DIR="));
}
