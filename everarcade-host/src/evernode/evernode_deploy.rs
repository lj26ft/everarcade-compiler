pub fn deploy(_package: &[u8], dry_run: bool) -> Result<String, String> {
    if dry_run {
        Ok("dry-run-instance".into())
    } else {
        Ok("live-instance-placeholder".into())
    }
}
