pub struct SimulationHealthMetric { pub continuity: &'static str, pub replay_continuity: &'static str, pub migration_readiness: &'static str }
pub fn health() -> SimulationHealthMetric { SimulationHealthMetric { continuity: "preserved", replay_continuity: "append-only", migration_readiness: "ready" } }
