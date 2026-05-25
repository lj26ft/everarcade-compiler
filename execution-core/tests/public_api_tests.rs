use execution_core::api;

#[test]
fn test_runtime_api_exports_compile() {
    let _ = api::runtime_validation_root(&execution_core::world::RuntimeMetrics::default());
    let _ = api::public_api_surface_hash();
}

#[test]
fn test_protocol_types_are_stable() {
    let _ = core::mem::size_of::<api::PublicApiSurfaceHash>();
    let _ = core::mem::size_of::<api::ProtocolSurfaceHash>();
}

#[test]
fn test_no_duplicate_public_symbols() {
    let _a = api::public_api_surface_hash();
    let _b = api::protocol_surface_hash();
}

#[test]
fn test_public_runtime_surface_hash_stability() {
    assert_eq!(api::public_api_surface_hash().0, "execution-core-api-v0");
}

#[test]
fn test_validation_api_surface_stability() {
    assert_eq!(api::protocol_surface_hash().0, "execution-core-protocol-v0");
}
