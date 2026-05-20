pub trait WorldHooks {
    fn on_world_tick(&mut self, _tick: u64) {}
    fn on_partition_migrate(&mut self, _entity_id: &str, _target_partition: &str) {}
}
