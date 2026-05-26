#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidationCorruption {
    pub reason: String,
}

pub fn detect_corruption(valid: bool, reason: &str) -> Result<(), ValidationCorruption> {
    if valid {
        Ok(())
    } else {
        Err(ValidationCorruption {
            reason: reason.to_string(),
        })
    }
}
