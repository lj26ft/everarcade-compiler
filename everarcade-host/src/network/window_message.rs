pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WindowMessage {
    pub window_id: Hash,
    pub replay_root: Hash,
}
