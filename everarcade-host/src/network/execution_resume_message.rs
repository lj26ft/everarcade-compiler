pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExecutionResumeMessage {
    pub partition_root: Hash,
    pub replay_root: Hash,
}

pub fn validate_resume_message(message: &ExecutionResumeMessage) -> bool {
    message.partition_root != [0u8; 32] && message.replay_root != [0u8; 32]
}
