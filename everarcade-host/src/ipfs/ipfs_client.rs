pub fn upload(_bytes: &[u8], dry_run: bool) -> Result<String, String> {
    if dry_run {
        Ok("dry-run-cid".into())
    } else {
        Ok("live-cid-placeholder".into())
    }
}
