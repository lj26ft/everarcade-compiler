use super::execution_message::ExecutionMessage;

pub fn validate_execution_transfer(message: &ExecutionMessage) -> bool {
    message.window_id != [0u8; 32] && message.execution_root != [0u8; 32]
}
