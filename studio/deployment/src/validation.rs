use crate::deployment::{federation, node, runtime};
pub fn validate_deployment_ux() -> crate::CreatorDiagnostic { crate::diagnostic("deployment-ux", &["runtime-package", "compatibility", "deployment-lineage", "status"] ) }
pub fn deployment_continuity() -> bool { let d = runtime::deploy_runtime("pkg", "everarcade-0.1"); d.compatible && node::node_status("node-a", &d.deployment_hash) == node::node_status("node-a", &d.deployment_hash) && federation::federation_lineage(&["node-a", "node-b"], &d.deployment_hash) == federation::federation_lineage(&["node-a", "node-b"], &d.deployment_hash) }
