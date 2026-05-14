use super::payload_validation::payload_valid;

pub fn validate_message(payload: &[u8]) -> Result<(), String> {
    if payload_valid(payload) {
        Ok(())
    } else {
        Err("invalid transport payload".to_string())
    }
}
