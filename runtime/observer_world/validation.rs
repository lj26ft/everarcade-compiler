pub fn reject_authority_mutation(authority_write: bool) -> Result<(), &'static str> {
    if authority_write {
        Err("observer world is reconstruction-only")
    } else {
        Ok(())
    }
}
