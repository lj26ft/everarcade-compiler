pub fn survives_infrastructure_change(
    node_changed: bool,
    verifier_changed: bool,
    runtime_changed: bool,
) -> bool {
    node_changed || verifier_changed || runtime_changed
}
