use crate::publishing::{package, preview, signing};
pub fn validate_publishing() -> crate::CreatorDiagnostic { crate::diagnostic("package-publishing-ux", &["package", "preview", "compatibility", "signing"] ) }
pub fn package_hash_equivalence() -> bool { let a = package::package_content("pkg", &["world", "assets"]); a.deterministic && a == package::package_content("pkg", &["world", "assets"]) && preview::preview_package(&a.package_hash) == preview::preview_package(&a.package_hash) && signing::sign_package(&a.package_hash, "creator") == signing::sign_package(&a.package_hash, "creator") }
