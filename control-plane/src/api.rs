use crate::operator::OperatorControlPlane;

pub fn deployment_status(control_plane: &OperatorControlPlane) -> String {
    format!(
        "deployments={}",
        control_plane.metrics().deployment.deployment_count
    )
}
pub fn runtime_health_status(control_plane: &OperatorControlPlane) -> String {
    format!(
        "federation_health={}",
        control_plane.topology.federation_health()
    )
}
