pub fn preview_package(package_hash: &str) -> String { crate::stable_hash(&["package-preview", package_hash]) }
