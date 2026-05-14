pub fn validate(expected: [u8; 32], observed: [u8; 32]) -> Result<(), &'static str> {
    if expected == observed {
        Ok(())
    } else {
        Err("civilization runtime divergence")
    }
}
