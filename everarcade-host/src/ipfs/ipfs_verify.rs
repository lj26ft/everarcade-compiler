pub fn verify_cid(cid: &str) -> bool {
    cid.starts_with("cid:")
}
