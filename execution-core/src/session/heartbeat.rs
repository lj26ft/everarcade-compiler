use super::presence::{Presence, PresenceState};

pub const DEFAULT_TIMEOUT_TICKS: u64 = 120;

pub fn record_heartbeat(presence: &mut Presence, tick: u64) {
    presence.last_heartbeat_tick = tick;
    presence.state = PresenceState::Online;
}

pub fn apply_timeout(presence: &mut Presence, tick: u64, timeout_ticks: u64) -> bool {
    if presence.is_active() && tick.saturating_sub(presence.last_heartbeat_tick) > timeout_ticks {
        presence.state = PresenceState::TimedOut;
        true
    } else {
        false
    }
}
