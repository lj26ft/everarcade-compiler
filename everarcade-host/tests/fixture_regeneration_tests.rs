use everarcade_host::fixture::{generate_fixture_bytes, generate_fixture_to_path};

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

#[test]
fn fixture_generation_is_deterministic() {
    let generated_a = generate_fixture_bytes().unwrap();
    let generated_b = generate_fixture_bytes().unwrap();
    assert_eq!(generated_a, generated_b);
}

#[test]
fn fixture_generation_to_path_matches_canonical_bytes() {
    let fixture_path = temp_path().join("civilization_package.bin");
    generate_fixture_to_path(&fixture_path).unwrap();

    let generated = generate_fixture_bytes().unwrap();
    let written = std::fs::read(fixture_path).unwrap();
    assert_eq!(generated, written);
}
