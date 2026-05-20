use std::time::Duration;

pub const DEFAULT_CONNECT_TIMEOUT: Duration = Duration::from_secs(3);
pub const DEFAULT_READ_TIMEOUT: Duration = Duration::from_secs(5);
pub const DEFAULT_SESSION_TIMEOUT: Duration = Duration::from_secs(15);

pub fn detect_session_timeout(last_activity_elapsed: Duration, timeout: Duration) -> bool {
    last_activity_elapsed > timeout
}
