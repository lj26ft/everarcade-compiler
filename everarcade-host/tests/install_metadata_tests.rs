#[test]
fn install_priority_order_is_documented_in_script() {
    let script = std::fs::read_to_string("scripts/install.sh").unwrap();
    assert!(script.contains("PREFIX=\"${PREFIX:-$PREFIX_DEFAULT}\""));
    assert!(script.contains("--prefix"));
    assert!(script.contains("SCRIPT_DIR="));
}
