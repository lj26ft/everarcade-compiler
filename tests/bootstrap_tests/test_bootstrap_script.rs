use std::path::Path;

#[test]
fn test_bootstrap_script_exists() {
    assert!(Path::new("scripts/everarcade_start.sh").exists());
}
