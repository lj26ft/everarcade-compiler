pub trait EntityHooks {
    fn on_entity_spawn(&mut self, _entity_id: &str) {}
    fn on_entity_update(&mut self, _entity_id: &str) {}
}
