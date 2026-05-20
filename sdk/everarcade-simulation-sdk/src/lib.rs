pub trait SimulationHooks {
    fn on_interaction(&mut self, _source_entity: &str, _target_entity: &str, _action: &str) {}
    fn on_partition_change(&mut self, _partition_id: &str) {}
}
