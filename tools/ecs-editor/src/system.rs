use crate::stable_hash;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SystemOrder { pub systems: Vec<String>, pub order_hash: String }

pub fn inspect_system_order(systems: &[&str]) -> SystemOrder {
    let ordered: Vec<String> = systems.iter().map(|s| (*s).to_owned()).collect();
    SystemOrder { systems: ordered, order_hash: stable_hash(systems) }
}

pub fn reject_nondeterministic_order(systems: &[&str]) -> Result<(), &'static str> {
    let mut sorted = systems.to_vec();
    sorted.sort_unstable();
    if systems == sorted.as_slice() { Ok(()) } else { Err("system ordering must be canonical and deterministic") }
}
