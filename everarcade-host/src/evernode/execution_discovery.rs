use super::{
    capacity_manifest::ExecutionCapacityManifest, operator_availability::OperatorAvailability,
};

pub fn discover_eligible_operators(
    manifests: &[ExecutionCapacityManifest],
    availability: &[OperatorAvailability],
) -> Vec<ExecutionCapacityManifest> {
    manifests
        .iter()
        .filter(|manifest| {
            availability.iter().any(|status| {
                status.operator_id == manifest.operator_id && status.available_windows > 0
            })
        })
        .cloned()
        .collect()
}
