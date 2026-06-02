use execution_core::arena_vanguard_live::{
    validate_multiplayer, ArenaVanguardGateway, GatewayHealth, PlayerAction, RuntimeHealth,
    RuntimeHostService,
};

fn joined_host() -> (RuntimeHostService, String, String) {
    let mut host = RuntimeHostService::start("arena-vanguard-live");
    let player = host.join("alice");
    let token = player.resume_token.clone();
    (host, player.player_id, token)
}

#[test]
fn test_runtime_host_start() {
    let host = RuntimeHostService::start("arena-vanguard-live");
    assert_eq!(host.runtime_health, RuntimeHealth::Healthy);
    assert_eq!(host.registry_entry().session_id, "arena-vanguard-live");
}

#[test]
fn test_gateway_runtime_attach() {
    let host = RuntimeHostService::start("arena-vanguard-live");
    let mut gateway = ArenaVanguardGateway::launch_and_attach(&host);
    assert_eq!(gateway.discover_runtime(&host), Some("arena-vanguard-live"));
    assert_eq!(gateway.health_check(&host), GatewayHealth::Healthy);
}

#[test]
fn test_player_join() {
    let mut host = RuntimeHostService::start("arena-vanguard-live");
    let player = host.join("alice");
    assert!(player.connected);
    assert_eq!(host.registry_entry().player_count, 1);
}

#[test]
fn test_player_spawn() {
    let mut host = RuntimeHostService::start("arena-vanguard-live");
    let player = host.join("alice");
    assert_eq!(player.position.zone, "Spawn Area");
    assert_eq!(player.inventory, vec!["starter blade".to_owned()]);
}

#[test]
fn test_player_move() {
    let (mut host, player_id, _) = joined_host();
    let feed = host
        .submit_action(&player_id, PlayerAction::Move { dx: 6, dy: 0 })
        .unwrap();
    let player = feed
        .players
        .iter()
        .find(|p| p.player_id == player_id)
        .unwrap();
    assert_eq!(player.position.zone, "Combat Area");
    assert_eq!(player.energy, 49);
}

#[test]
fn test_player_attack() {
    let (mut host, player_id, _) = joined_host();
    let feed = host
        .submit_action(
            &player_id,
            PlayerAction::Attack {
                enemy_id: "enemy-raider-1".to_owned(),
            },
        )
        .unwrap();
    assert_eq!(feed.enemies[0].health, 75);
    assert_eq!(feed.players[0].xp, 35);
}

#[test]
fn test_player_loot() {
    let (mut host, player_id, _) = joined_host();
    let feed = host
        .submit_action(
            &player_id,
            PlayerAction::Interact {
                item_id: "field cache".to_owned(),
            },
        )
        .unwrap();
    assert!(feed.players[0]
        .inventory
        .contains(&"field cache".to_owned()));
}

#[test]
fn test_state_stream() {
    let (host, _, _) = joined_host();
    let feed = host.world_state_feed();
    assert!(feed.world_zones.contains(&"Spawn Area".to_owned()));
    assert!(feed.world_zones.contains(&"Combat Area".to_owned()));
    assert_eq!(feed.enemies[0].status, "hostile");
}

#[test]
fn test_disconnect_reconnect() {
    let (mut host, player_id, token) = joined_host();
    host.disconnect(&player_id).unwrap();
    let restored = host.reconnect(&token).unwrap();
    assert!(restored.connected);
    assert_eq!(restored.player_id, player_id);
}

#[test]
fn test_session_restore() {
    let (mut host, player_id, token) = joined_host();
    host.submit_action(&player_id, PlayerAction::Move { dx: 3, dy: 0 })
        .unwrap();
    host.submit_action(
        &player_id,
        PlayerAction::Interact {
            item_id: "field cache".to_owned(),
        },
    )
    .unwrap();
    let saved = host.disconnect(&player_id).unwrap();
    let restored = host.reconnect(&token).unwrap();
    assert_eq!(restored.position, saved.position);
    assert_eq!(restored.inventory, saved.inventory);
    assert_eq!(restored.level, saved.level);
}

#[test]
fn test_gateway_recovery() {
    let mut host = RuntimeHostService::start("arena-vanguard-live");
    let mut gateway = ArenaVanguardGateway::launch_and_attach(&host);
    host.stop();
    assert_eq!(gateway.health_check(&host), GatewayHealth::Recovering);
    host.recover();
    gateway.reattach(&host);
    assert_eq!(gateway.health_check(&host), GatewayHealth::Healthy);
}

#[test]
fn test_runtime_recovery() {
    let (mut host, player_id, token) = joined_host();
    let before = host.replay_equivalence_root();
    host.stop();
    host.recover();
    let restored = host.reconnect(&token).unwrap();
    assert_eq!(host.runtime_health, RuntimeHealth::Healthy);
    assert_eq!(restored.player_id, player_id);
    assert_ne!(before, host.replay_equivalence_root());
}

#[test]
fn test_multiplayer_two_players() {
    assert!(validate_multiplayer(2));
}

#[test]
fn test_multiplayer_five_players() {
    assert!(validate_multiplayer(5));
}

#[test]
fn test_multiplayer_ten_players() {
    assert!(validate_multiplayer(10));
}

#[test]
fn test_live_session_complete() {
    let (mut host, player_id, token) = joined_host();
    host.submit_action(&player_id, PlayerAction::Move { dx: 6, dy: 0 })
        .unwrap();
    host.submit_action(
        &player_id,
        PlayerAction::Attack {
            enemy_id: "enemy-raider-1".to_owned(),
        },
    )
    .unwrap();
    host.submit_action(
        &player_id,
        PlayerAction::Interact {
            item_id: "field cache".to_owned(),
        },
    )
    .unwrap();
    let saved = host.disconnect(&player_id).unwrap();
    let restored = host.reconnect(&token).unwrap();
    let status = host.status();
    assert_eq!(restored.position, saved.position);
    assert!(status.replay_growth >= 6);
    assert_eq!(status.players_online, 1);
}
