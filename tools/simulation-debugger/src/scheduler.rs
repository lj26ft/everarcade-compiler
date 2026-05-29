use crate::stable_hash;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SchedulerView { pub ordered_systems: Vec<String>, pub order_hash: String }

pub fn inspect_scheduler_order(systems: &[&str]) -> SchedulerView { SchedulerView { ordered_systems: systems.iter().map(|s| (*s).to_owned()).collect(), order_hash: stable_hash(systems) } }
