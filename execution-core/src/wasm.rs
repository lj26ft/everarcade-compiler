use crate::abi::{ExecutionInput, StateDiff, StateChange};

pub fn execute_logic(input: ExecutionInput) -> StateDiff {
    // deterministic execution logic
    let change = StateChange {
        key: input.action,
        before: "0".into(),
        after: input.payload,
    };

    StateDiff {
        changes: vec![change],
    }
}
