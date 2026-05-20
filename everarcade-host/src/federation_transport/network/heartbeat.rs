use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
pub struct HeartbeatState {
    pub interval: Duration,
    pub last_seen: Instant,
    pub last_continuity_update: Instant,
    pub last_latency: Option<Duration>,
    pub last_checkpoint: u64,
}

impl HeartbeatState {
    pub fn is_timed_out(&self, now: Instant, grace: Duration) -> bool {
        now.duration_since(self.last_seen) > self.interval + grace
    }
}

pub fn send_heartbeat(state: &mut HeartbeatState, now: Instant, peer_response_at: Instant) {
    state.last_seen = now;
    state.last_latency = Some(peer_response_at.saturating_duration_since(now));
}

pub fn verify_peer_liveness(state: &HeartbeatState, now: Instant, grace: Duration) -> bool {
    !state.is_timed_out(now, grace)
}
