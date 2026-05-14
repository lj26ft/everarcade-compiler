use execution_core::{
    simulation::{convergence_simulation::simulate_convergence, node::SimulatedNode},
    sync::{SyncRequest, SyncResponse, SyncStatus},
};

#[test]
fn node_simulation_runs() {
    let node = SimulatedNode {
        node_id: [7; 32],
        sync_status: SyncStatus {
            state_root: [0; 32],
            replay_root: [0; 32],
            receipt_root: [0; 32],
            next_index: 0,
        },
        checkpoint: None,
        receipts: vec![],
    };
    let ok = simulate_convergence(
        &node,
        SyncRequest {
            local_state_root: [0; 32],
            local_replay_root: [0; 32],
            local_receipt_root: [0; 32],
            from_index: 0,
            to_index: Some(0),
        },
        SyncResponse {
            checkpoint: None,
            receipts: vec![],
            state_proofs: vec![],
            receipt_proofs: vec![],
            replay_proof: None,
        },
    );
    assert!(ok);
}
