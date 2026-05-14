pub type Hash = [u8; 32];
pub fn message_signature_is_present(signature_root: Hash) -> bool {
    signature_root != [0; 32]
}
