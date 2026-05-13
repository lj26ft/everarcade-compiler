pub fn submit_anchor(_payload: &[u8], dry_run: bool) -> Result<String, String> {
    if dry_run {
        Ok("dry-run-tx".into())
    } else {
        Ok("live-tx-placeholder".into())
    }
}
