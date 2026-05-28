#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeHealthCheck {
    pub replay_continuity_ok: bool,
    pub storage_writable: bool,
    pub storage_restorable: bool,
    pub network_listener_active: bool,
    pub observer_stream_active: bool,
    pub recovery_ok: bool,
    pub non_authoritative_mode: bool,
}

impl RuntimeHealthCheck {
    pub fn ready() -> Self {
        Self {
            replay_continuity_ok: true,
            storage_writable: true,
            storage_restorable: true,
            network_listener_active: true,
            observer_stream_active: true,
            recovery_ok: true,
            non_authoritative_mode: true,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.replay_continuity_ok
            && self.storage_writable
            && self.storage_restorable
            && self.network_listener_active
            && self.observer_stream_active
            && self.recovery_ok
            && self.non_authoritative_mode
    }
}
