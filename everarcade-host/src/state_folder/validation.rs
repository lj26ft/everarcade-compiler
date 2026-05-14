use super::layout::required_paths;
use std::path::Path;
pub fn validate(base: &Path) -> bool {
    required_paths(base).iter().all(|p| p.exists())
}
