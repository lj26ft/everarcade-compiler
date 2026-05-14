pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SignedIdentityManifest {
    pub manifest_root: Hash,
    pub signer_root: Hash,
    pub signature_root: Hash,
}
