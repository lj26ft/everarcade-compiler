pub fn can_retry(attempts: u32, max_attempts: u32) -> bool {
    attempts < max_attempts
}
