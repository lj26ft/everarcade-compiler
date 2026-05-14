pub fn reject_if_invalid(valid: bool) -> Result<(), &'static str> {
    if valid {
        Ok(())
    } else {
        Err("deterministic governance rejection")
    }
}
