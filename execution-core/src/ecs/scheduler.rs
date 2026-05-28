use super::system::DeterministicSystem;

pub fn deterministic_system_order(
    mut systems: Vec<DeterministicSystem>,
) -> Vec<DeterministicSystem> {
    systems.sort_by(|a, b| a.id.cmp(&b.id).then_with(|| a.component.cmp(&b.component)));
    systems
}

pub fn deterministic_entity_order(mut ids: Vec<String>) -> Vec<String> {
    ids.sort();
    ids
}
