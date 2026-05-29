use crate::{content_packager, diagnostic, CreatorDiagnostic};

pub fn validate_content_packaging() -> CreatorDiagnostic { diagnostic("content-packaging-validation", &["package", "archive", "signing", "runtime"] ) }

pub fn content_package_hash_equivalence(package_id: &str, assets: &[&str]) -> bool { content_packager::package::package_content(package_id, assets) == content_packager::package::package_content(package_id, assets) }

pub fn reject_incompatible_content(compatible: bool) -> Result<(), &'static str> { if compatible { Ok(()) } else { Err("incompatible content rejected") } }
