pub fn live_enabled() -> bool {
    cfg!(feature = "ipfs-live")
}
