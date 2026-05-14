#[test]
fn recovery_name_is_stable() {
    let module = "recovery";
    let parts: Vec<&str> = module.split('_').collect();
    assert!(
        !parts.is_empty(),
        "module name should have at least one segment"
    );
    assert!(
        parts.iter().all(|p| !p.is_empty()),
        "module segments must be non-empty"
    );
    let rebuilt = parts.join("_");
    assert_eq!(
        rebuilt, module,
        "splitting and rejoining module name must be deterministic"
    );
}
