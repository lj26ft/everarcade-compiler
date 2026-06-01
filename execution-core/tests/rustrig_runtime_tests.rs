use contract_api::protocol_records::{fields, ProtocolRecord, XrplRecord};
use contract_api::rustrig::RustrigContext;
use execution_core::rustrig_runtime::composition::{execute_pipeline, PipelineStep};
use execution_core::rustrig_runtime::{
    ExecutionRequest, RecordApplication, RustrigKernel, RustrigRuntimeError,
};

fn ctx() -> RustrigContext {
    RustrigContext {
        world_root: "world:0".into(),
        replay_root: "replay:0".into(),
        checkpoint_root: "checkpoint:0".into(),
        actor_id: "hero".into(),
        tick: 1,
        input_hash: "input:stable".into(),
        protocol_version: "everarcade-protocol-1".into(),
    }
}
fn run(
    id: &str,
    payload: &[(&str, String)],
) -> execution_core::rustrig_runtime::kernel::KernelExecution {
    let mut kernel = RustrigKernel::default();
    kernel
        .execute(ExecutionRequest {
            rustrig_id: id.into(),
            version: "1.0.0".into(),
            context: ctx(),
            payload: fields(payload),
        })
        .unwrap()
}

#[test]
fn test_rustrig_kernel_executes() {
    assert_eq!(
        run("world.spawn_entity", &[("entity", "p1".into())])
            .receipt
            .record_count,
        1
    );
}
#[test]
fn test_rustrig_registry_lookup() {
    let k = RustrigKernel::default();
    assert!(k.registry.lookup("combat.apply_damage").is_ok());
    assert!(k.registry.lookup("missing").is_err());
}
#[test]
fn test_rustrig_output_records_apply() {
    let e = run("world.spawn_entity", &[("entity", "p1".into())]);
    assert!(e.applied[0].authoritative);
}
#[test]
fn test_combat_rustrig_applies_damage() {
    let e = run(
        "combat.apply_damage",
        &[("target", "slime".into()), ("amount", "7".into())],
    );
    assert!(matches!(e.output.records[0], ProtocolRecord::Combat(_)));
}
#[test]
fn test_inventory_rustrig_transfer() {
    assert!(matches!(
        run("inventory.transfer_item", &[("item", "key".into())])
            .output
            .records[0],
        ProtocolRecord::Inventory(_)
    ));
}
#[test]
fn test_quest_rustrig_progression() {
    assert!(matches!(
        run("quests.advance_objective", &[("quest", "q1".into())])
            .output
            .records[0],
        ProtocolRecord::Quest(_)
    ));
}
#[test]
fn test_dialogue_rustrig_choice() {
    assert!(matches!(
        run(
            "dialogue.complete_dialogue",
            &[("conversation", "elder".into())]
        )
        .output
        .records[0],
        ProtocolRecord::Dialogue(_)
    ));
}
#[test]
fn test_economy_rustrig_ledger() {
    assert!(matches!(
        run("economy.create_ledger_entry", &[("amount", "10".into())])
            .output
            .records[0],
        ProtocolRecord::Economy(_)
    ));
}
#[test]
fn test_world_rustrig_spawn_move() {
    let mut k = RustrigKernel::default();
    for id in ["world.spawn_entity", "world.move_entity"] {
        k.execute(ExecutionRequest {
            rustrig_id: id.into(),
            version: "1.0.0".into(),
            context: ctx(),
            payload: fields(&[("entity", "p1".into())]),
        })
        .unwrap();
    }
    assert!(k.state.world.contains_key("p1"));
}
#[test]
fn test_crafting_rustrig_outputs() {
    assert!(matches!(
        run("crafting.craft_item", &[("item", "sword".into())])
            .output
            .records[0],
        ProtocolRecord::Inventory(_)
    ));
}
#[test]
fn test_faction_rustrig_membership() {
    assert!(matches!(
        run("factions.assign_member", &[("target", "guild".into())])
            .output
            .records[0],
        ProtocolRecord::Entity(_)
    ));
}
#[test]
fn test_movement_rustrig_bounds() {
    assert!(matches!(
        run(
            "movement.move_actor",
            &[("x", "1".into()), ("y", "2".into())]
        )
        .output
        .records[0],
        ProtocolRecord::World(_)
    ));
}
#[test]
fn test_interaction_rustrig_trigger() {
    assert!(matches!(
        run(
            "interaction.activate_trigger",
            &[("trigger", "door".into())]
        )
        .output
        .records[0],
        ProtocolRecord::World(_)
    ));
}
#[test]
fn test_xrpl_rustrig_intent_only() {
    let e = run("xrpl.create_anchor_intent", &[("asset", "XRP".into())]);
    assert!(!e.applied[0].authoritative);
    assert!(matches!(e.output.records[0], ProtocolRecord::XrplIntent(_)));
}
#[test]
fn test_deployment_rustrig_intent_only() {
    let e = run(
        "deployment.create_deployment_intent",
        &[("package", "pkg".into())],
    );
    assert!(!e.applied[0].authoritative);
    assert!(matches!(
        e.output.records[0],
        ProtocolRecord::DeploymentIntent(_)
    ));
}
#[test]
fn test_rustrig_composition_pipeline() {
    let mut k = RustrigKernel::default();
    let c = execute_pipeline(
        &mut k,
        "p",
        "1.0.0",
        ctx(),
        vec![
            PipelineStep {
                id: "world.spawn_entity".into(),
                payload: fields(&[("entity", "p".into())]),
            },
            PipelineStep {
                id: "combat.apply_damage".into(),
                payload: fields(&[("target", "p".into())]),
            },
        ],
    )
    .unwrap();
    assert_eq!(c.receipts.len(), 2);
}
#[test]
fn test_arena_vanguard_uses_real_rustrigs() {
    let mut k = RustrigKernel::default();
    for id in [
        "world.spawn_entity",
        "world.move_entity",
        "combat.apply_damage",
        "inventory.add_item",
        "quests.advance_objective",
        "dialogue.complete_dialogue",
        "economy.create_ledger_entry",
        "replay.emit_event",
        "runtime.checkpoint",
        "runtime.recover",
    ] {
        k.execute(ExecutionRequest {
            rustrig_id: id.into(),
            version: "1.0.0".into(),
            context: ctx(),
            payload: fields(&[
                ("entity", "player".into()),
                ("target", "player".into()),
                ("item", "medal".into()),
                ("quest", "arena".into()),
                ("conversation", "coach".into()),
            ]),
        })
        .unwrap();
    }
    assert!(k.replay.events.len() >= 10);
}
#[test]
fn test_rustrig_replay_equivalence() {
    let a = run("combat.apply_damage", &[("target", "p".into())]).receipt;
    let b = run("combat.apply_damage", &[("target", "p".into())]).receipt;
    assert_eq!(a.record_root, b.record_root);
}
#[test]
fn test_rustrig_checkpoint_recovery() {
    let mut k = RustrigKernel::default();
    let cp = k
        .execute(ExecutionRequest {
            rustrig_id: "runtime.checkpoint".into(),
            version: "1.0.0".into(),
            context: ctx(),
            payload: fields(&[]),
        })
        .unwrap()
        .receipt
        .checkpoint_root;
    let rv = k
        .execute(ExecutionRequest {
            rustrig_id: "runtime.recover".into(),
            version: "1.0.0".into(),
            context: ctx(),
            payload: fields(&[]),
        })
        .unwrap()
        .receipt
        .checkpoint_root;
    assert!(!cp.is_empty() && !rv.is_empty());
}
#[test]
fn test_authority_mutation_rejection() {
    let mut state = Default::default();
    let rec = ProtocolRecord::Xrpl(XrplRecord::new("submit", "tx", fields(&[])));
    assert!(matches!(
        RecordApplication::apply(&mut state, &rec),
        Err(RustrigRuntimeError::AuthorityMutationRejected(_))
    ));
}
