use sha2::{
    Digest,
    Sha256,
};

use crate::{
    receipt::ExecutionReceipt,
    state::{
        State,
        StateChange,
    },
};

fn sha256(
    input: &str,
) -> String {
    let mut hasher =
        Sha256::new();

    hasher.update(input.as_bytes());

    hex::encode(
        hasher.finalize(),
    )
}

pub fn hash_state(
    state: &State,
) -> String {
    sha256(
        &serde_json::to_string(state)
            .unwrap(),
    )
}

pub fn hash_execution(
    changes: &Vec<StateChange>,
) -> String {
    sha256(
        &serde_json::to_string(changes)
            .unwrap(),
    )
}

pub fn hash_receipt(
    receipt: &ExecutionReceipt,
) -> String {
    sha256(
        &serde_json::to_string(receipt)
            .unwrap(),
    )
}
