use crate::stable_hash;

pub fn visualize_runtime_execution(ticks: &[&str]) -> String { stable_hash(ticks) }
