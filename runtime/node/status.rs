#![allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeNodeStatus { pub running: bool, pub replay_tip: u64, pub continuity_root: String, pub non_authoritative: bool }
impl RuntimeNodeStatus { pub fn ready(&self) -> bool { self.running && self.non_authoritative && !self.continuity_root.is_empty() } }
