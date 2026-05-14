pub type Hash = [u8; 32];
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SignedCheckpointMessage {
    pub checkpoint_root: Hash,
    pub signer_root: Hash,
    pub signature_root: Hash,
}
