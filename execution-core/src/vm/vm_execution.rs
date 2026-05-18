use sha2::{Digest, Sha256};

use super::vm_input::VmExecutionInput;
use super::vm_output::VmExecutionOutput;
use super::vm_receipt::{compute_vm_receipt_root, VmExecutionReceipt};
use everarcade_abi::StateChange;

pub const REPLAY_ROOT_STATE_KEY: &str = "__replay_root__";

pub fn genesis_replay_root_value() -> Vec<u8> {
    hex::encode(Sha256::digest([])).into_bytes()
}

pub fn execute_vm_boundary(input: &VmExecutionInput) -> (VmExecutionReceipt, VmExecutionOutput) {
    let execution_root: [u8; 32] = Sha256::digest(
        [
            input.package_manifest_root.as_slice(),
            input.civilization_root.as_slice(),
            input.payload_root.as_slice(),
        ]
        .concat(),
    )
    .into();

    let next_replay_root: [u8; 32] =
        Sha256::digest([input.pre_state_root.as_slice(), execution_root.as_slice()].concat())
            .into();

    let anchor_root: [u8; 32] = Sha256::digest(
        [
            next_replay_root.as_slice(),
            input.checkpoint_root.as_slice(),
        ]
        .concat(),
    )
    .into();

    let state_diff = vec![StateChange {
        key: REPLAY_ROOT_STATE_KEY.to_string(),
        before: hex::encode(input.prior_replay_root_value),
        after: hex::encode(next_replay_root),
    }];

    let mut receipt = VmExecutionReceipt {
        receipt_id: [0; 32],
        package_root: input.package_manifest_root,
        prior_replay_root: input.pre_state_root,
        next_replay_root,
        execution_root,
        checkpoint_root: input.checkpoint_root,
        anchor_root,
        state_diff,
    };
    receipt.receipt_id = compute_vm_receipt_root(&receipt);

    let output = VmExecutionOutput {
        vm_receipt_root: receipt.receipt_id,
        execution_root: receipt.execution_root,
        replay_root: receipt.next_replay_root,
        checkpoint_root: receipt.checkpoint_root,
        external_anchor_root: receipt.anchor_root,
    };

    (receipt, output)
}
