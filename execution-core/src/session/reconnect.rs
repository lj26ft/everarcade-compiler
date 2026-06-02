use super::presence::{Presence, PresenceState};

pub fn reconnect(presence: &mut Presence, tick: u64) {
    presence.last_heartbeat_tick = tick;
    presence.state = PresenceState::Online;
}
