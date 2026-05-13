pub fn live_enabled() -> bool {
    cfg!(feature = "evernode-live")
}
