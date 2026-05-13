use super::ipfs_client::upload;
pub fn upload_artifact(bytes: &[u8], dry_run: bool) -> Result<String, String> {
    upload(bytes, dry_run)
}
