pub fn is_final(status: &str) -> bool {
    matches!(status, "validated" | "failed")
}
