pub type Hash = [u8; 32];
pub fn invalid_signature_detected(valid: bool) -> bool {
    !valid
}
