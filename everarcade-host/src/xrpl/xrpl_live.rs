pub fn live_enabled() -> bool {
    cfg!(feature = "xrpl-live")
}
