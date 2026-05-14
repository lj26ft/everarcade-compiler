use std::path::Path;

pub fn verify_package_artifacts(state: &Path) -> bool {
    state.join("packages").exists()
}
