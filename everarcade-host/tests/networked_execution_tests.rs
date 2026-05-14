use everarcade_host::network::{
    execution_message::ExecutionMessage, execution_transfer::validate_execution_transfer,
};

#[test]
fn invalid_execution_message_rejected() {
    let invalid = ExecutionMessage {
        window_id: [0; 32],
        execution_root: [1; 32],
    };
    assert!(!validate_execution_transfer(&invalid));
}
