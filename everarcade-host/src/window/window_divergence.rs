pub fn detect_window_divergence(
    expected: [u8; 32],
    observed: [u8; 32],
) -> Result<(), &'static str> {
    if expected == observed {
        Ok(())
    } else {
        Err("window divergence")
    }
}
