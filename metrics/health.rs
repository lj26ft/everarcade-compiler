//! Health metrics for sovereign runtime deployment.

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RuntimeHealthSnapshot {
    pub continuity_ok: bool,
    pub recovery_ok: bool,
    pub observers_in_sync: bool,
    pub active_peers: usize,
}

impl RuntimeHealthSnapshot {
    pub fn healthy(&self) -> bool {
        self.continuity_ok && self.recovery_ok && self.observers_in_sync
    }
}
