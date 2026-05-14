pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WindowContinuity {
    pub root: Hash,
}
