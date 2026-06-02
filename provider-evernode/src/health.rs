use control_plane::health::RuntimeHealth;

pub fn collect_runtime_health(runtime_id: &str, process_state: &str) -> RuntimeHealth {
    let mut health = RuntimeHealth::healthy(runtime_id, 1);
    health.network_status = "federated".into();
    health.recovery_status = "idle".into();
    if process_state != "running" {
        health.runtime_alive = false;
    }
    health.evaluate();
    health
}
