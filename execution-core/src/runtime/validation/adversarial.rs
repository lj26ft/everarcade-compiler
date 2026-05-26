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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuntimeSecurityValidation {
    pub replay_injection_rejected: bool,
    pub replay_truncation_rejected: bool,
    pub archive_corruption_rejected: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuntimeSecurityViolation {
    pub code: String,
    pub reason: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuntimeSecurityResult {
    pub passed: bool,
    pub violations: Vec<RuntimeSecurityViolation>,
}
