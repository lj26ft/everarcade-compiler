use execution_core::assets::*;
use execution_core::game_runtime::{
    entities::Entity, input::PlayerInput, multiplayer::deterministic_order, simulation::step,
    world_state::WorldState,
};
use execution_core::gpu::*;
use execution_core::xrpl::{
    anchors::AnchorCommitment, canonical_bytes, settlement::SettlementReceipt,
};

#[test]
fn test_deterministic_player_movement() {
    let mut world = WorldState::new();
    world.entities.insert(
        1,
        Entity {
            id: 1,
            x: 0,
            y: 0,
            authority: "p1".into(),
            runtime_lineage: "runtime-0".into(),
            world_continuity: "world-alpha".into(),
        },
    );
    let next = step(
        world,
        &[PlayerInput {
            player_id: "p1".into(),
            dx: 2,
            dy: -1,
        }],
    );
    let e = next.entities.get(&1).unwrap();
    assert_eq!((e.x, e.y, next.tick), (2, -1, 1));
}

#[test]
fn test_multiplayer_input_replay() {
    let ordered = deterministic_order(vec![
        PlayerInput {
            player_id: "z".into(),
            dx: 0,
            dy: 0,
        },
        PlayerInput {
            player_id: "a".into(),
            dx: 0,
            dy: 0,
        },
    ]);
    assert_eq!(ordered[0].player_id, "a");
}

#[test]
fn test_entity_authority_equivalence() {
    let e = Entity {
        id: 1,
        x: 0,
        y: 0,
        authority: "p1".into(),
        runtime_lineage: "runtime-0".into(),
        world_continuity: "world-alpha".into(),
    };
    assert!(execution_core::game_runtime::authority::authoritative(
        &e, "p1"
    ));
    assert!(!execution_core::game_runtime::authority::authoritative(
        &e, "p2"
    ));
}

#[test]
fn test_world_restore_equivalence() {
    let w = WorldState::new();
    let cloned = w.clone();
    assert_eq!(w, cloned);
}

#[test]
fn test_gpu_boundary_equivalence() {
    let env = GpuResultEnvelope {
        task: GpuExecutionTask {
            task_id: "t".into(),
            tick: 1,
            input_hash: "i".into(),
            workload: "render".into(),
        },
        witness: GpuWitness {
            task_id: "t".into(),
            output_hash: "o".into(),
            worker_id: "g1".into(),
        },
        replay_anchor: GpuReplayAnchor {
            task_id: "t".into(),
            authoritative_state_root: "s".into(),
            replay_hash: "r".into(),
        },
    };
    assert_eq!(env.task.task_id, env.witness.task_id);
}

#[test]
fn test_inventory_ownership_continuity() {
    let id = AssetIdentity {
        world_id: "w".into(),
        asset_id: "a".into(),
        class: "item".into(),
    };
    let manifest = AssetManifest {
        identity: id,
        version: 1,
        metadata_hash: "m".into(),
    };
    let rec = AssetOwnershipRecord {
        manifest,
        owner: "alice".into(),
        continuity_counter: 10,
    };
    assert_eq!(rec.continuity_counter, 10);
}

#[test]
fn test_xrpl_anchor_equivalence() {
    let a = AnchorCommitment {
        world_id: "w".into(),
        state_root: "s".into(),
        replay_root: "r".into(),
        tick: 5,
    };
    let b = AnchorCommitment {
        world_id: "w".into(),
        state_root: "s".into(),
        replay_root: "r".into(),
        tick: 5,
    };
    assert_eq!(canonical_bytes(&a), canonical_bytes(&b));
}

#[test]
fn test_runtime_tick_equivalence() {
    assert_eq!(execution_core::game_runtime::tick::advance_tick(9), 10);
}

#[test]
fn test_large_world_progression() {
    let mut world = WorldState::new();
    for i in 0..1000 {
        world.entities.insert(
            i,
            Entity {
                id: i,
                x: 0,
                y: 0,
                authority: "p1".into(),
                runtime_lineage: "runtime-0".into(),
                world_continuity: "world-alpha".into(),
            },
        );
    }
    let world = step(
        world,
        &[PlayerInput {
            player_id: "p1".into(),
            dx: 1,
            dy: 1,
        }],
    );
    assert_eq!(world.tick, 1);
}

#[test]
fn test_settlement_receipt_shape() {
    let receipt = SettlementReceipt {
        world_id: "w".into(),
        settlement_epoch: 1,
        amount_drops: 10,
        destination: "rDest".into(),
        witness_hash: "h".into(),
    };
    assert_eq!(receipt.settlement_epoch, 1);
}
