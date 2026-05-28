use super::{priority::EntityPriority, tick::deterministic_order, validation::validate_schedule};
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntitySchedulerRuntime {
    pub tick: u64,
    pub last_order: Vec<String>,
    pub replay_tip: String,
}
impl EntitySchedulerRuntime {
    pub fn new() -> Self {
        Self {
            tick: 0,
            last_order: vec![],
            replay_tip: "scheduler:replay:0".into(),
        }
    }
    pub fn schedule(&mut self, items: Vec<EntityPriority>) -> Result<Vec<String>, &'static str> {
        let ordered = deterministic_order(items)
            .into_iter()
            .map(|i| i.entity_id)
            .collect::<Vec<_>>();
        self.tick += 1;
        self.replay_tip = format!("scheduler:replay:{}:{}", self.tick, ordered.join(","));
        self.last_order = ordered.clone();
        if validate_schedule(self) {
            Ok(ordered)
        } else {
            Err("scheduling divergence rejected")
        }
    }
}
