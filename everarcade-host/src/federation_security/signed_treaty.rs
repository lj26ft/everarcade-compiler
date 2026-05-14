pub type Hash = [u8; 32];
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SignedTreaty { pub treaty_root: Hash, pub signer_root: Hash, pub signature_root: Hash }
