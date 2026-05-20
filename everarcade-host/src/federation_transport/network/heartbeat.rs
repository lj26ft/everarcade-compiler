use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
pub struct HeartbeatState {
    pub interval: Duration,
    pub last_seen: Instant,
}

impl HeartbeatState {
    pub fn is_timed_out(&self, now: Instant, grace: Duration) -> bool {
        now.duration_since(self.last_seen) > self.interval + grace
    }
}
