use std::collections::{BTreeMap, BTreeSet};

use contract_api::protocol_records::RecordFields;
use contract_api::rustrig::{RustrigContext, RustrigOutput};

use super::error::{Result, RustrigRuntimeError};
use super::executor::stable_hash;

type ExecFn = fn(&RustrigContext, &RecordFields) -> contract_api::protocol_records::ProtocolRecord;

#[derive(Clone)]
pub struct RegisteredRustrig {
    pub id: &'static str,
    pub version: &'static str,
    pub allowed_categories: BTreeSet<&'static str>,
    pub execute: ExecFn,
}

#[derive(Clone, Default)]
pub struct RustrigRegistry {
    rigs: BTreeMap<&'static str, RegisteredRustrig>,
}

impl RustrigRegistry {
    pub fn with_builtins() -> Self {
        let mut registry = Self::default();
        for (id, cats, exec) in builtins() {
            registry.register(id, "1.0.0", cats, exec);
        }
        registry
    }
    pub fn register(
        &mut self,
        id: &'static str,
        version: &'static str,
        cats: &[&'static str],
        execute: ExecFn,
    ) {
        self.rigs.insert(
            id,
            RegisteredRustrig {
                id,
                version,
                allowed_categories: cats.iter().copied().collect(),
                execute,
            },
        );
    }
    pub fn lookup(&self, id: &str) -> Result<&RegisteredRustrig> {
        self.rigs
            .get(id)
            .ok_or_else(|| RustrigRuntimeError::UnknownRustrig(id.to_string()))
    }
    pub fn validate_version(&self, id: &str, version: &str) -> Result<()> {
        let rig = self.lookup(id)?;
        if rig.version == version {
            Ok(())
        } else {
            Err(RustrigRuntimeError::AbiIncompatible {
                rustrig_id: id.to_string(),
                reason: format!("expected version {}, got {version}", rig.version),
            })
        }
    }
    pub fn ids(&self) -> Vec<&'static str> {
        self.rigs.keys().copied().collect()
    }
    pub fn execute(
        &self,
        id: &str,
        ctx: &RustrigContext,
        payload: &RecordFields,
    ) -> Result<RustrigOutput> {
        let rig = self.lookup(id)?;
        let record = (rig.execute)(ctx, payload);
        let records = vec![record];
        Ok(RustrigOutput {
            rustrig_id: id.to_string(),
            version: rig.version.to_string(),
            output_hash: stable_hash(&records),
            records,
        })
    }
}

fn get(p: &RecordFields, k: &str, default: &str) -> String {
    p.get(k).cloned().unwrap_or_else(|| default.to_string())
}
fn n(p: &RecordFields, k: &str, default: u64) -> u64 {
    p.get(k).and_then(|v| v.parse().ok()).unwrap_or(default)
}
fn i(p: &RecordFields, k: &str, default: i64) -> i64 {
    p.get(k).and_then(|v| v.parse().ok()).unwrap_or(default)
}

use contract_api::protocol_records::{
    fields, CombatRecord, DeploymentIntentRecord, DialogueRecord, EconomyRecord, EntityRecord,
    InventoryRecord, ProtocolRecord, QuestRecord, ReplayRecord, WorldRecord, XrplIntentRecord,
};
fn pr(
    category: &str,
    action: &'static str,
    ctx: &RustrigContext,
    p: &RecordFields,
) -> ProtocolRecord {
    let actor = get(p, "actor", &ctx.actor_id);
    let target = get(p, "target", &actor);
    let tick = n(p, "tick", ctx.tick);
    match category {
        "world" => ProtocolRecord::World(WorldRecord::new(
            action,
            get(p, "entity", &target),
            fields(&[
                ("world", get(p, "world", &ctx.world_root)),
                ("x", i(p, "x", 0).to_string()),
                ("y", i(p, "y", 0).to_string()),
                ("actor", actor),
                ("tick", tick.to_string()),
            ]),
        )),
        "entity" => ProtocolRecord::Entity(EntityRecord::new(
            action,
            target,
            fields(&[
                ("actor", actor),
                ("amount", n(p, "amount", 0).to_string()),
                ("tick", tick.to_string()),
            ]),
        )),
        "combat" => ProtocolRecord::Combat(CombatRecord::new(
            action,
            target,
            fields(&[
                ("actor", actor),
                ("amount", n(p, "amount", 0).to_string()),
                ("status", get(p, "status", "none")),
                ("tick", tick.to_string()),
            ]),
        )),
        "inventory" => ProtocolRecord::Inventory(InventoryRecord::new(
            action,
            actor,
            fields(&[
                ("item", get(p, "item", "item")),
                ("quantity", n(p, "quantity", 1).to_string()),
                ("counterparty", get(p, "counterparty", "runtime")),
                ("slot", get(p, "slot", "bag")),
                ("tick", tick.to_string()),
            ]),
        )),
        "quest" => ProtocolRecord::Quest(QuestRecord::new(
            action,
            get(p, "quest", "quest"),
            fields(&[
                ("player", actor),
                ("step", get(p, "step", "0")),
                ("reward", get(p, "reward", "none")),
                ("tick", tick.to_string()),
            ]),
        )),
        "dialogue" => ProtocolRecord::Dialogue(DialogueRecord::new(
            action,
            get(p, "conversation", "conversation"),
            fields(&[
                ("actor", actor),
                ("node", get(p, "node", "start")),
                ("choice", get(p, "choice", "none")),
                ("tick", tick.to_string()),
            ]),
        )),
        "economy" => ProtocolRecord::Economy(EconomyRecord::new(
            action,
            actor,
            fields(&[
                ("asset", get(p, "asset", "coin")),
                ("amount", n(p, "amount", 0).to_string()),
                ("counterparty", get(p, "counterparty", "runtime")),
                ("tick", tick.to_string()),
            ]),
        )),
        "xrpl-intent" => ProtocolRecord::XrplIntent(XrplIntentRecord::new(
            action,
            actor,
            fields(&[
                ("asset", get(p, "asset", "XRP")),
                ("amount", n(p, "amount", 0).to_string()),
                ("destination", get(p, "destination", "rRuntime")),
                ("submission", "runtime-bridge-only".to_string()),
                ("tick", tick.to_string()),
            ]),
        )),
        "deployment-intent" => ProtocolRecord::DeploymentIntent(DeploymentIntentRecord::new(
            action,
            get(p, "package", "package"),
            fields(&[
                ("target", get(p, "target", "evernode")),
                ("version", get(p, "version", "1")),
                ("execution", "runtime-orchestrator-only".to_string()),
                ("tick", tick.to_string()),
            ]),
        )),
        "replay" => ProtocolRecord::Replay(ReplayRecord::new(
            action,
            actor,
            fields(&[
                ("tick", tick.to_string()),
                ("input_hash", ctx.input_hash.clone()),
            ]),
        )),
        _ => ProtocolRecord::Entity(EntityRecord::new("unknown", actor, fields(&[]))),
    }
}
macro_rules! builtin {
    ($fn:ident,$cat:literal,$action:literal) => {
        fn $fn(ctx: &RustrigContext, p: &RecordFields) -> ProtocolRecord {
            pr($cat, $action, ctx, p)
        }
    };
}
builtin!(spawn_entity, "world", "spawn-entity");
builtin!(move_entity, "world", "move-entity");
builtin!(apply_damage, "combat", "apply-damage");
builtin!(add_item, "inventory", "add-item");
builtin!(transfer_item, "inventory", "transfer-item");
builtin!(advance_objective, "quest", "advance-objective");
builtin!(complete_dialogue, "dialogue", "complete-dialogue");
builtin!(create_ledger_entry, "economy", "create-ledger-entry");
builtin!(emit_replay_event, "replay", "emit-replay-event");
builtin!(checkpoint, "replay", "checkpoint");
builtin!(recover, "replay", "recover");
builtin!(craft_item, "inventory", "craft-item");
builtin!(assign_member, "entity", "assign-member");
builtin!(move_actor, "world", "move-actor");
builtin!(activate_trigger, "world", "activate-trigger");
builtin!(create_xrpl_intent, "xrpl-intent", "create-anchor-intent");
builtin!(
    create_deployment_intent,
    "deployment-intent",
    "create-deployment-intent"
);
fn builtins() -> Vec<(&'static str, &'static [&'static str], ExecFn)> {
    vec![
        ("world.spawn_entity", &["world"], spawn_entity),
        ("world.move_entity", &["world"], move_entity),
        ("combat.apply_damage", &["combat"], apply_damage),
        ("inventory.add_item", &["inventory"], add_item),
        ("inventory.transfer_item", &["inventory"], transfer_item),
        ("quests.advance_objective", &["quest"], advance_objective),
        (
            "dialogue.complete_dialogue",
            &["dialogue"],
            complete_dialogue,
        ),
        (
            "economy.create_ledger_entry",
            &["economy"],
            create_ledger_entry,
        ),
        ("replay.emit_event", &["replay"], emit_replay_event),
        ("runtime.checkpoint", &["replay"], checkpoint),
        ("runtime.recover", &["replay"], recover),
        ("crafting.craft_item", &["inventory", "economy"], craft_item),
        (
            "factions.assign_member",
            &["entity", "world", "economy"],
            assign_member,
        ),
        ("movement.move_actor", &["world", "entity"], move_actor),
        (
            "interaction.activate_trigger",
            &["world", "inventory", "quest"],
            activate_trigger,
        ),
        (
            "xrpl.create_anchor_intent",
            &["xrpl-intent"],
            create_xrpl_intent,
        ),
        (
            "deployment.create_deployment_intent",
            &["deployment-intent"],
            create_deployment_intent,
        ),
    ]
}
