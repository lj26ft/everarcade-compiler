use super::presence::{Presence, PresenceState};

pub fn disconnect(presence: &mut Presence, tick: u64) {
    presence.last_heartbeat_tick = tick;
    presence.state = PresenceState::Disconnected;
}

pub fn leave(presence: &mut Presence, tick: u64) {
    presence.last_heartbeat_tick = tick;
    presence.state = PresenceState::Left;
}
