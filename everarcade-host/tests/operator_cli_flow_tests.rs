fn temp_path() -> std::path::PathBuf {
    let mut p = std::env::temp_dir();
    let n = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    p.push(format!("everarcade-test-{}", n));
    std::fs::create_dir_all(&p).unwrap();
    p
}
use std::process::Command;
#[test]
fn cli_flow() {
    let d = temp_path();
    let s = d.as_path().to_str().unwrap();
    for a in [
        ["init", "--state", s].as_slice(),
        [
            "run",
            "--package",
            &format!(
                "{}/tests/fixtures/civilization_package.bin",
                env!("CARGO_MANIFEST_DIR")
            ),
            "--state",
            s,
        ]
        .as_slice(),
        ["verify", "--state", s].as_slice(),
        ["status", "--state", s].as_slice(),
        ["anchor-intent", "--state", s].as_slice(),
    ] {
        assert!(Command::new(env!("CARGO_BIN_EXE_everarcade-host"))
            .args(a)
            .status()
            .unwrap()
            .success());
    }
}
