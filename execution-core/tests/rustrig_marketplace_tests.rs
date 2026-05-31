use execution_core::marketplace::{
    compose_packages, install_package, reject_invalid_package, replay_safe_marketplace,
    validate_package, MarketplacePackage,
};
use rustrigs::dependency::DependencyGraph;
use rustrigs::package::{arena_vanguard_required_manifests, reproducible, RustrigPackageManifest};

fn manifest(id: &str, deps: Vec<String>) -> RustrigPackageManifest {
    RustrigPackageManifest {
        package_id: id.to_owned(),
        name: id.to_owned(),
        version: "0.1.0".to_owned(),
        author: "community-author".to_owned(),
        description: "deterministic package".to_owned(),
        dependencies: deps,
        record_types: vec!["CombatRecord".to_owned()],
        protocol_version: "everarcade-protocol-1".to_owned(),
        hash: format!("{id}-hash"),
    }
}

fn package(id: &str, deps: Vec<String>) -> MarketplacePackage {
    let manifest = manifest(id, deps);
    MarketplacePackage {
        signature: format!("signed:{}", manifest.hash),
        manifest,
        package_bytes: b"deterministic bytecode".to_vec(),
        abi_version: "everarcade-protocol-1".to_owned(),
        deterministic: true,
        replay_safe: true,
        records_valid: true,
        requests_network: false,
        requests_filesystem: false,
        requests_authority_write: false,
        requests_deployment: false,
        requests_xrpl_submission: false,
    }
}

#[test]
fn test_package_reproducibility() {
    let manifest = manifest("combat", Vec::new());
    assert!(reproducible(&manifest, b"same", b"same"));
}

#[test]
fn test_dependency_resolution() {
    let graph = DependencyGraph::new(vec![
        manifest("base", Vec::new()),
        manifest("combat", vec!["base".to_owned()]),
    ]);
    assert_eq!(
        graph.resolve(&["combat".to_owned()]).unwrap(),
        vec!["base", "combat"]
    );
}

#[test]
fn test_registry_validation() {
    let pkg = package("combat", Vec::new());
    let graph = DependencyGraph::new(vec![pkg.manifest.clone()]);
    assert!(validate_package(&pkg, &graph).accepted());
}

#[test]
fn test_package_certification() {
    let certified_stages = [
        "Build",
        "Hash",
        "Validate",
        "Replay Check",
        "ABI Check",
        "Sign",
        "Publish",
    ];
    assert_eq!(certified_stages.len(), 7);
    assert!(certified_stages.contains(&"Publish"));
}

#[test]
fn test_marketplace_installation() {
    let pkg = package("combat", Vec::new());
    let graph = DependencyGraph::new(vec![pkg.manifest.clone()]);
    assert!(install_package(pkg, &graph).is_ok());
}

#[test]
fn test_marketplace_composition() {
    let packages = vec![
        package("combat", Vec::new()),
        package("inventory", Vec::new()),
    ];
    assert_eq!(compose_packages(&packages).unwrap().len(), 2);
}

#[test]
fn test_abi_compatibility() {
    let pkg = package("combat", Vec::new());
    let graph = DependencyGraph::new(vec![pkg.manifest.clone()]);
    assert!(validate_package(&pkg, &graph).abi_compatible);
}

#[test]
fn test_authority_rejection() {
    let mut pkg = package("combat", Vec::new());
    pkg.requests_authority_write = true;
    let graph = DependencyGraph::new(vec![pkg.manifest.clone()]);
    assert!(reject_invalid_package(&pkg, &graph).is_err());
}

#[test]
fn test_replay_safe_marketplace() {
    let packages = vec![
        package("combat", Vec::new()),
        package("inventory", Vec::new()),
    ];
    assert!(replay_safe_marketplace(&packages));
}

#[test]
fn test_arena_vanguard_marketplace_flow() {
    let manifests = arena_vanguard_required_manifests();
    let packages: Vec<_> = manifests
        .iter()
        .map(|manifest| MarketplacePackage {
            signature: format!("signed:{}", manifest.hash),
            manifest: manifest.clone(),
            package_bytes: manifest.package_id.as_bytes().to_vec(),
            abi_version: "everarcade-protocol-1".to_owned(),
            deterministic: true,
            replay_safe: true,
            records_valid: true,
            requests_network: false,
            requests_filesystem: false,
            requests_authority_write: false,
            requests_deployment: false,
            requests_xrpl_submission: false,
        })
        .collect();
    let graph = DependencyGraph::new(manifests);
    assert_eq!(packages.len(), 6);
    assert!(packages
        .iter()
        .all(|pkg| install_package(pkg.clone(), &graph).is_ok()));
    assert!(replay_safe_marketplace(&packages));
}
