use execution_core::state::{apply_diff, CanonicalState};
use execution_core::vm::{
    execute_vm_boundary, genesis_replay_root_value, VmExecutionInput, REPLAY_ROOT_STATE_KEY,
};

fn decode_hex32(v: &[u8]) -> [u8; 32] {
    let s = std::str::from_utf8(v).unwrap();
    let bytes = hex::decode(s).unwrap();
    bytes.as_slice().try_into().unwrap()
}

#[test]
fn test_replay_root_raw_hash_representation() {
    let input = VmExecutionInput {
        package_manifest_root: [1; 32],
        civilization_root: [1; 32],
        pre_state_root: [2; 32],
        prior_replay_root_value: [3; 32],
        checkpoint_root: [4; 32],
        payload_root: [5; 32],
    };
    let (receipt, _) = execute_vm_boundary(&input);
    assert_eq!(decode_hex32(receipt.state_diff[0].before.as_bytes()), [3; 32]);
    assert_eq!(decode_hex32(receipt.state_diff[0].after.as_bytes()), receipt.next_replay_root);
}

#[test]
fn test_replay_root_state_encoding_roundtrip() {
    let mut state = CanonicalState::default();
    state
        .entries
        .insert(REPLAY_ROOT_STATE_KEY.as_bytes().to_vec(), genesis_replay_root_value());

    let encoded = state.entries.get(REPLAY_ROOT_STATE_KEY.as_bytes()).unwrap();
    let decoded = decode_hex32(encoded);
    assert_eq!(hex::encode(decoded).into_bytes(), *encoded);
}

#[test]
fn test_replay_root_diff_encoding_consistency() {
    let mut state = CanonicalState::default();
    state
        .entries
        .insert(REPLAY_ROOT_STATE_KEY.as_bytes().to_vec(), genesis_replay_root_value());
    let prior = decode_hex32(state.entries.get(REPLAY_ROOT_STATE_KEY.as_bytes()).unwrap());

    let (receipt, _) = execute_vm_boundary(&VmExecutionInput {
        package_manifest_root: [9; 32],
        civilization_root: [9; 32],
        pre_state_root: state.root(),
        prior_replay_root_value: prior,
        checkpoint_root: [7; 32],
        payload_root: [7; 32],
    });

    apply_diff(&mut state, &receipt.state_diff).unwrap();
    let state_value = state.entries.get(REPLAY_ROOT_STATE_KEY.as_bytes()).unwrap();
    assert_eq!(state_value, &receipt.state_diff[0].after.as_bytes().to_vec());
}
