use serde::{Deserialize, Serialize};

use crate::{
    abi::ABI_VERSION,
    hashing::{
        hash_execution,
        hash_receipt,
        hash_state,
    },
    receipt::ExecutionReceipt,
    registry::ContractRegistry,
    state::{
        State,
        StateChange,
    },
};

use std::collections::BTreeMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExecutionNode {
    pub id: String,
    pub contract: String,
    pub payload: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExecutionPlan {
    pub nodes: Vec<ExecutionNode>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VmInput {
    pub state: State,
    pub plan: ExecutionPlan,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VmOutput {
    pub updated_state: State,
    pub receipt: ExecutionReceipt,
}

pub fn execute_vm(
    input: VmInput,
) -> VmOutput {
    let mut state = input.state.clone();

    let previous_state_root =
        hash_state(&state);

    let mut changes: Vec<StateChange> =
        vec![];

    for node in input.plan.nodes.iter() {
        let contract =
            ContractRegistry::execute(
                &node.contract,
            );

        contract.execute(
            &node.payload,
            &mut state,
            &mut changes,
        );
    }

    let new_state_root =
        hash_state(&state);

    let execution_root =
        hash_execution(&changes);

    let mut receipt = ExecutionReceipt {
        previous_state_root,
        new_state_root,
        execution_root,
        receipt_hash: String::new(),
        abi_version:
            ABI_VERSION.to_string(),
    };

    receipt.receipt_hash =
        hash_receipt(&receipt);

    VmOutput {
        updated_state: state,
        receipt,
    }
}
