use std::{
    fs,
    path::{Path, PathBuf},
};

use execution_core::{
    canonical::{generate_execution_manifest, receipt_hash, save_manifest},
    lineage::{save_lineage, ExecutionLineageChain, ExecutionLineageRecord},
    operator::{recover_world, OperatorRecoveryInput, OperatorRecoveryOutput},
    persistence::{package_store, receipt_store},
    state::{apply_diff, decode_checkpoint, encode_checkpoint, CanonicalState},
    vm::{
        execute_vm_boundary, genesis_replay_root_value, VmExecutionInput, VmExecutionReceipt,
        REPLAY_ROOT_STATE_KEY,
    },
};

#[derive(Clone)]
pub struct CounterWorldFixture {
    pub package_bytes: Vec<u8>,
    pub checkpoint_0: Vec<u8>,
    pub checkpoint_1: Vec<u8>,
    pub checkpoint_2: Vec<u8>,
    pub receipt_1: VmExecutionReceipt,
    pub receipt_2: VmExecutionReceipt,
    pub lineage: ExecutionLineageChain,
}

fn h(v: u8) -> [u8; 32] {
    [v; 32]
}

pub fn generate_counter_world_fixture() -> CounterWorldFixture {
    let package_bytes = vec![1u8; 64];
    let package_root = package_store::package_root(&package_bytes);
    let mut state0 = CanonicalState::default();
    state0.entries.insert(
        REPLAY_ROOT_STATE_KEY.as_bytes().to_vec(),
        genesis_replay_root_value(),
    );
    let state0_root = state0.root();
    let checkpoint_0 = encode_checkpoint(&state0).unwrap();

    let (receipt_1, _next_1) = execute_vm_boundary(&VmExecutionInput {
        package_manifest_root: package_root,
        civilization_root: package_root,
        pre_state_root: state0_root,
        prior_replay_root_value: state0_root,
        checkpoint_root: h(31),
        payload_root: h(31),
    });
    let mut state1 = state0.clone();
    let replay_root_before = state0
        .entries
        .get(REPLAY_ROOT_STATE_KEY.as_bytes())
        .cloned()
        .unwrap();
    assert_eq!(
        Some(&replay_root_before),
        state1.entries.get(REPLAY_ROOT_STATE_KEY.as_bytes())
    );
    assert_eq!(
        receipt_1.state_diff[0].before.as_bytes().to_vec(),
        replay_root_before
    );
    apply_diff(&mut state1, &receipt_1.state_diff).unwrap();
    let state1_root = state1.root();
    assert_eq!(receipt_1.prior_replay_root, state0_root);
    assert_eq!(receipt_1.next_replay_root, state1_root);
    let checkpoint_1 = encode_checkpoint(&state1).unwrap();

    let (receipt_2, _next_2) = execute_vm_boundary(&VmExecutionInput {
        package_manifest_root: package_root,
        civilization_root: package_root,
        pre_state_root: state1_root,
        prior_replay_root_value: state1_root,
        checkpoint_root: h(32),
        payload_root: h(32),
    });
    let mut state2 = state1.clone();
    apply_diff(&mut state2, &receipt_2.state_diff).unwrap();
    let state2_root = state2.root();
    assert_eq!(receipt_2.prior_replay_root, state1_root);
    assert_eq!(receipt_2.next_replay_root, state2_root);
    let checkpoint_2 = encode_checkpoint(&state2).unwrap();

    let lineage = ExecutionLineageChain {
        world_id: package_root,
        package_root,
        records: vec![
            ExecutionLineageRecord {
                sequence: 1,
                previous_execution_id: None,
                execution_id: receipt_1.execution_root,
                pre_state_root: state0_root,
                post_state_root: state1_root,
                receipt_hash: receipt_1.receipt_id,
                package_root,
            },
            ExecutionLineageRecord {
                sequence: 2,
                previous_execution_id: Some(receipt_1.execution_root),
                execution_id: receipt_2.execution_root,
                pre_state_root: state1_root,
                post_state_root: state2_root,
                receipt_hash: receipt_2.receipt_id,
                package_root,
            },
        ],
    };
    assert_eq!(state0_root, lineage.records[0].pre_state_root);
    assert_eq!(state1_root, lineage.records[0].post_state_root);
    assert_eq!(state1_root, lineage.records[1].pre_state_root);
    assert_eq!(state2_root, lineage.records[1].post_state_root);

    CounterWorldFixture {
        package_bytes,
        checkpoint_0,
        checkpoint_1,
        checkpoint_2,
        receipt_1,
        receipt_2,
        lineage,
    }
}

pub fn generate_two_step_lineage_fixture() -> CounterWorldFixture {
    generate_counter_world_fixture()
}

pub fn repo_counter_world_fixture_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../everarcade-host/tests/fixtures/counter_world")
}

pub fn persist_counter_world_fixture(dir: &Path, fixture: &CounterWorldFixture) {
    fs::create_dir_all(dir).unwrap();
    fs::write(dir.join("world.wasm"), &fixture.package_bytes).unwrap();
    fs::write(dir.join("checkpoint_0.bin"), &fixture.checkpoint_0).unwrap();
    fs::write(dir.join("checkpoint_1.bin"), &fixture.checkpoint_1).unwrap();
    fs::write(dir.join("checkpoint_2.bin"), &fixture.checkpoint_2).unwrap();
    receipt_store::save_receipt(&dir.join("receipt_1.bin"), &fixture.receipt_1).unwrap();
    receipt_store::save_receipt(&dir.join("receipt_2.bin"), &fixture.receipt_2).unwrap();
    save_lineage(&dir.join("lineage.bin"), &fixture.lineage).unwrap();

    let manifest = generate_execution_manifest(
        fixture.lineage.package_root,
        receipt_hash(&fixture.receipt_2),
        &fixture.lineage,
        decode_checkpoint(&fixture.checkpoint_0).unwrap().root(),
        fixture.lineage.records[1].post_state_root,
    );
    save_manifest(&dir.join("manifest.bin"), &manifest).unwrap();
}

pub fn generate_operator_recovery_fixture(dir: &Path) -> OperatorRecoveryOutput {
    let fixture = generate_counter_world_fixture();
    persist_counter_world_fixture(dir, &fixture);
    recover_world(OperatorRecoveryInput {
        package_path: dir.join("world.wasm"),
        checkpoint_path: dir.join("checkpoint_0.bin"),
        lineage_path: dir.join("lineage.bin"),
        receipt_paths: vec![dir.join("receipt_1.bin"), dir.join("receipt_2.bin")],
        descriptor_output_path: dir.join("recovery_descriptor.bin"),
    })
    .unwrap()
}

pub fn ensure_repo_counter_world_fixtures() {
    let dir = repo_counter_world_fixture_dir();
    let fixture = generate_counter_world_fixture();
    persist_counter_world_fixture(&dir, &fixture);
    let _ = recover_world(OperatorRecoveryInput {
        package_path: dir.join("world.wasm"),
        checkpoint_path: dir.join("checkpoint_0.bin"),
        lineage_path: dir.join("lineage.bin"),
        receipt_paths: vec![dir.join("receipt_1.bin"), dir.join("receipt_2.bin")],
        descriptor_output_path: dir.join("recovery_descriptor.bin"),
    })
    .unwrap();
}
