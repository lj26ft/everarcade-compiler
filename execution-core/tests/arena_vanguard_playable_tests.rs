use execution_core::session::{ArenaVanguardRuntime, PlayerId};

fn joined_runtime(
    seed: &str,
) -> (
    ArenaVanguardRuntime,
    PlayerId,
    execution_core::session::CharacterId,
) {
    let mut runtime = ArenaVanguardRuntime::new();
    let (_session_id, character_id) = runtime.join(seed);
    (runtime, PlayerId::new(seed), character_id)
}

#[test]
fn test_player_join() {
    let (runtime, player_id, _) = joined_runtime("alice");
    assert!(runtime.presences.contains_key(&player_id));
    assert_eq!(runtime.telemetry().player_count, 1);
}

#[test]
fn test_player_spawn() {
    let (runtime, _, character_id) = joined_runtime("alice");
    let character = runtime.characters.get(&character_id).unwrap();
    assert_eq!((character.x, character.y), (0, 0));
    assert!(character.alive);
}

#[test]
fn test_player_move() {
    let (mut runtime, _, character_id) = joined_runtime("alice");
    let records = runtime.move_player(&character_id, 2, 1);
    let character = runtime.characters.get(&character_id).unwrap();
    assert_eq!((character.x, character.y), (2, 1));
    assert!(!records.is_empty());
}

#[test]
fn test_player_attack() {
    let (mut runtime, _, character_id) = joined_runtime("alice");
    let enemy_id = "enemy-Goblin-1";
    let before = runtime.enemies.get(enemy_id).unwrap().health;
    let records = runtime.attack_enemy(&character_id, enemy_id);
    assert!(runtime.enemies.get(enemy_id).unwrap().health < before);
    assert!(!records.is_empty());
}

#[test]
fn test_enemy_kill() {
    let (mut runtime, _, character_id) = joined_runtime("alice");
    let enemy_id = "enemy-Goblin-1";
    while runtime.enemies.get(enemy_id).unwrap().alive {
        runtime.attack_enemy(&character_id, enemy_id);
    }
    assert!(!runtime.enemies.get(enemy_id).unwrap().alive);
}

#[test]
fn test_loot_collection() {
    let (mut runtime, _, character_id) = joined_runtime("alice");
    let records = runtime.collect_loot(&character_id, "loot-potion-1");
    let character = runtime.characters.get(&character_id).unwrap();
    assert!(character
        .inventory
        .iter()
        .any(|item| item.item == "health potion" && item.quantity == 1));
    assert!(!records.is_empty());
}

#[test]
fn test_inventory_persistence() {
    let (mut runtime, player_id, character_id) = joined_runtime("alice");
    runtime.collect_loot(&character_id, "loot-sword-1");
    runtime.disconnect(&player_id);
    let restored = runtime.restore_checkpoint();
    let character = restored.characters.get(&character_id).unwrap();
    assert!(character
        .inventory
        .iter()
        .any(|item| item.item == "iron sword"));
}

#[test]
fn test_xp_gain() {
    let (mut runtime, _, character_id) = joined_runtime("alice");
    runtime.grant_xp(&character_id, 50);
    assert_eq!(
        runtime.characters.get(&character_id).unwrap().experience,
        50
    );
}

#[test]
fn test_level_up() {
    let (mut runtime, _, character_id) = joined_runtime("alice");
    runtime.grant_xp(&character_id, 100);
    let character = runtime.characters.get(&character_id).unwrap();
    assert_eq!(character.level, 2);
    assert!(character.attack > 20);
}

#[test]
fn test_disconnect_reconnect() {
    let (mut runtime, player_id, character_id) = joined_runtime("alice");
    runtime.collect_loot(&character_id, "loot-gold-1");
    runtime.disconnect(&player_id);
    assert_eq!(runtime.telemetry().player_count, 0);
    let reconnected = runtime.reconnect(&player_id).unwrap();
    assert_eq!(reconnected, character_id);
    assert_eq!(runtime.telemetry().player_count, 1);
}

#[test]
fn test_multiplayer_join() {
    let mut runtime = ArenaVanguardRuntime::new();
    for i in 0..10 {
        runtime.join(&format!("player-{i}"));
    }
    assert_eq!(runtime.telemetry().session_count, 10);
    assert_eq!(runtime.telemetry().player_count, 10);
}

#[test]
fn test_runtime_session_equivalence() {
    let (mut a, _, character_id_a) = joined_runtime("alice");
    let (mut b, _, character_id_b) = joined_runtime("alice");
    a.move_player(&character_id_a, 1, 0);
    b.move_player(&character_id_b, 1, 0);
    assert!(a.replay_equivalent(&b));
}

#[test]
fn test_replay_equivalence() {
    let (mut runtime, _, character_id) = joined_runtime("alice");
    runtime.move_player(&character_id, 1, 0);
    runtime.attack_enemy(&character_id, "enemy-Goblin-1");
    let replay = runtime.replay.clone();
    assert_eq!(replay, runtime.replay);
}

#[test]
fn test_checkpoint_restore() {
    let (mut runtime, player_id, character_id) = joined_runtime("alice");
    runtime.move_player(&character_id, 3, 0);
    runtime.disconnect(&player_id);
    let restored = runtime.restore_checkpoint();
    assert!(runtime.replay_equivalent(&restored));
}

#[test]
fn test_vertical_slice_complete() {
    let (mut runtime, player_id, character_id) = joined_runtime("alice");
    runtime.move_player(&character_id, 5, 0);
    while runtime.enemies.get("enemy-Goblin-1").unwrap().alive {
        runtime.attack_enemy(&character_id, "enemy-Goblin-1");
    }
    let loot_id = runtime
        .loot
        .keys()
        .find(|id| id.starts_with("loot-enemy-Goblin-1"))
        .unwrap()
        .clone();
    runtime.collect_loot(&character_id, &loot_id);
    runtime.grant_xp(&character_id, 100);
    runtime.disconnect(&player_id);
    let mut restored = runtime.restore_checkpoint();
    let character = restored.characters.get(&character_id).unwrap();
    assert!(character.level >= 2);
    assert!(character
        .inventory
        .iter()
        .any(|item| item.item == "gold" && item.quantity >= 10));
    assert!(restored.reconnect(&player_id).is_some());
}
