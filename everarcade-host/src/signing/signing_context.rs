pub type Hash = [u8; 32];
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SigningContext {
    pub signer_root: Hash,
    pub federation_scope_root: Option<Hash>,
}
