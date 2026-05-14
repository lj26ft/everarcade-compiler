pub type Hash = [u8; 32];
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QuarantineRecord { pub peer_root: Hash, pub reason_root: Hash }
