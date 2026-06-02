use super::player::{PlayerId, SessionId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PresenceState {
    Online,
    Disconnected,
    TimedOut,
    Left,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Presence {
    pub player_id: PlayerId,
    pub session_id: SessionId,
    pub state: PresenceState,
    pub joined_tick: u64,
    pub last_heartbeat_tick: u64,
}

impl Presence {
    pub fn online(player_id: PlayerId, session_id: SessionId, tick: u64) -> Self {
        Self {
            player_id,
            session_id,
            state: PresenceState::Online,
            joined_tick: tick,
            last_heartbeat_tick: tick,
        }
    }

    pub fn is_active(&self) -> bool {
        matches!(self.state, PresenceState::Online)
    }
}
