use std::path::Path; use super::layout::required_paths;
pub fn validate(base: &Path) -> bool { required_paths(base).iter().all(|p| p.exists()) }
