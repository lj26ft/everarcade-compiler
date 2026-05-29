use crate::stable_hash;

pub fn inspect_deployment_state(project_id: &str, package_hash: &str) -> String { stable_hash(&["deployment", project_id, package_hash]) }
