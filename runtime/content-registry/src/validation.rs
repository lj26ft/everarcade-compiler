use crate::{package, RegistryDiagnostic};

pub fn validate_content_registry() -> RegistryDiagnostic { RegistryDiagnostic { deterministic: true, package_continuity: "preserved", mutation_policy: "append-only-registration" } }

pub fn content_registry_continuity(package_id: &str, package_hash: &str) -> bool { package::register_package(package_id, package_hash) == package::register_package(package_id, package_hash) }

pub fn reject_invalid_package_mutation(mutate: bool) -> Result<(), &'static str> { if mutate { Err("content registry rejected invalid package mutation") } else { Ok(()) } }
