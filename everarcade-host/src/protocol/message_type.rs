#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MessageType {
    Receipt,
    Checkpoint,
    Proof,
    Manifest,
}
