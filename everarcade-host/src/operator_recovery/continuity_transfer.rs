pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContinuityTransfer {
    pub assignment_id: Hash,
    pub from_operator: Hash,
    pub to_operator: Hash,
    pub checkpoint_root: Hash,
}
