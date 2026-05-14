pub type Hash = [u8; 32];
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SignatureEnvelope { pub payload_root: Hash, pub signer_root: Hash, pub signature_root: Hash }
