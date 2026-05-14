pub type Hash = [u8; 32];
use super::envelope::ProtocolEnvelope; #[derive(Clone, Debug, PartialEq, Eq)]
pub struct SignedEnvelope { pub envelope: ProtocolEnvelope, pub signer_root: Hash, pub signature_root: Hash }
