use contract_api::{
    abi_v1::{canonical_hash, record_hash},
    protocol_records::ProtocolRecord,
};
use rustrigs::{
    combat::{ApplyDamage, CombatInput},
    deployment::{CreateDeploymentIntent, DeploymentIntentInput},
    dialogue::{DialogueInput, StartDialogue},
    inventory::{AddItem, InventoryInput},
    quests::{QuestInput, StartQuest},
    world::{SpawnEntity, WorldInput},
    xrpl::{CreateXrplIntent, XrplIntentInput},
    Rustrig,
};

fn combat_input() -> CombatInput {
    CombatInput {
        actor: "hero".into(),
        target: "slime".into(),
        amount: 7,
        status: "none".into(),
        tick: 1,
    }
}

#[test]
fn test_rustrig_determinism() {
    assert_eq!(
        ApplyDamage::execute(combat_input()),
        ApplyDamage::execute(combat_input())
    );
}

#[test]
fn test_rustrig_replay_equivalence() {
    let a = ProtocolRecord::Combat(ApplyDamage::execute(combat_input()));
    let b = ProtocolRecord::Combat(ApplyDamage::execute(combat_input()));
    assert_eq!(record_hash(&a), record_hash(&b));
}

#[test]
fn test_rustrig_composition() {
    let damage = ApplyDamage::execute(combat_input());
    let quest = StartQuest::execute(QuestInput {
        player: damage.fields["actor"].clone(),
        quest: "first-blood".into(),
        step: damage.action,
        reward: "xp:5".into(),
        tick: 2,
    });
    assert_eq!(quest.action, "start-quest");
    assert_eq!(quest.fields["player"], "hero");
}

#[test]
fn test_record_hash_equivalence() {
    let record = ProtocolRecord::Combat(ApplyDamage::execute(combat_input()));
    assert_eq!(record_hash(&record), canonical_hash(&record));
}

#[test]
fn test_inventory_rig() {
    let record = AddItem::execute(InventoryInput {
        owner: "hero".into(),
        item: "potion".into(),
        quantity: 2,
        counterparty: "loot".into(),
        slot: "bag".into(),
        tick: 3,
    });
    assert_eq!(record.action, "add-item");
    assert_eq!(record.fields["quantity"], "2");
}

#[test]
fn test_combat_rig() {
    let record = ApplyDamage::execute(combat_input());
    assert_eq!(record.action, "apply-damage");
    assert_eq!(record.subject, "slime");
}

#[test]
fn test_quest_rig() {
    let record = StartQuest::execute(QuestInput {
        player: "hero".into(),
        quest: "q1".into(),
        step: "0".into(),
        reward: "xp".into(),
        tick: 4,
    });
    assert_eq!(record.action, "start-quest");
}

#[test]
fn test_dialogue_rig() {
    let record = StartDialogue::execute(DialogueInput {
        actor: "hero".into(),
        conversation: "elder".into(),
        node: "start".into(),
        choice: "none".into(),
        tick: 5,
    });
    assert_eq!(record.action, "start-dialogue");
}

#[test]
fn test_world_rig() {
    let record = SpawnEntity::execute(WorldInput {
        entity: "npc:1".into(),
        world: "w".into(),
        x: 1,
        y: 2,
        owner: "runtime".into(),
        faction: "neutral".into(),
        tick: 6,
    });
    assert_eq!(record.action, "spawn-entity");
    assert_eq!(record.fields["x"], "1");
}

#[test]
fn test_xrpl_intent_rig() {
    let record = CreateXrplIntent::execute(XrplIntentInput {
        account: "rAlice".into(),
        intent: "payment".into(),
        asset: "XRP".into(),
        amount: 10,
        destination: "rBob".into(),
        tick: 7,
    });
    assert_eq!(record.action, "create-xrpl-intent");
    assert_eq!(record.fields["submission"], "runtime-bridge-only");
}

#[test]
fn test_deployment_intent_rig() {
    let record = CreateDeploymentIntent::execute(DeploymentIntentInput {
        package: "pkg".into(),
        target: "evernode".into(),
        version: "1".into(),
        tick: 8,
    });
    assert_eq!(record.action, "create-deployment-intent");
    assert_eq!(record.fields["execution"], "runtime-orchestrator-only");
}
