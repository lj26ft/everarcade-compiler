use execution_core::arena_vanguard_live::{
    validate_multiplayer, ArenaVanguardGateway, BrowserGatewayMessage, BrowserWebSocketGateway,
    GatewayHealth, PlayerAction, RuntimeHealth, RuntimeHostService,
};

#[test]
fn test_runtime_host_start() {
    let host = RuntimeHostService::start("arena-vanguard-live");
    assert_eq!(host.runtime_health, RuntimeHealth::Healthy);
}

#[test]
fn test_gateway_start() {
    let host = RuntimeHostService::start("arena-vanguard-live");
    let mut gateway = ArenaVanguardGateway::launch_and_attach(&host);
    assert_eq!(gateway.health_check(&host), GatewayHealth::Healthy);
}

#[test]
fn test_browser_connect() {
    let mut host = RuntimeHostService::start("arena-vanguard-live");
    let mut gateway = BrowserWebSocketGateway::open("browser-a");
    gateway.handle(
        &mut host,
        BrowserGatewayMessage::Join {
            player_seed: "alice".to_owned(),
        },
    );
    assert_eq!(host.status().connected_browsers, 1);
}

#[test]
fn test_two_browser_players() {
    assert!(validate_multiplayer(2));
}

#[test]
fn test_five_browser_players() {
    assert!(validate_multiplayer(5));
}

#[test]
fn test_ten_browser_players() {
    assert!(validate_multiplayer(10));
}

#[test]
fn test_runtime_recovery() {
    let mut host = RuntimeHostService::start("arena-vanguard-live");
    let session = host.join("alice");
    host.stop();
    host.recover();
    let restored = host.reconnect(&session.resume_token).unwrap();
    assert_eq!(host.runtime_health, RuntimeHealth::Healthy);
    assert_eq!(restored.player_id, session.player_id);
}

#[test]
fn test_browser_reconnect() {
    let mut host = RuntimeHostService::start("arena-vanguard-live");
    let mut gateway = BrowserWebSocketGateway::open("browser-a");
    let joined = gateway
        .handle(
            &mut host,
            BrowserGatewayMessage::Join {
                player_seed: "alice".to_owned(),
            },
        )
        .unwrap();
    let (player_id, token) = match joined {
        execution_core::arena_vanguard_live::BrowserGatewayEvent::Joined { session, .. } => {
            (session.player_id, session.resume_token)
        }
        _ => panic!("join expected"),
    };
    gateway
        .handle(&mut host, BrowserGatewayMessage::Leave { player_id })
        .unwrap();
    gateway
        .handle(
            &mut host,
            BrowserGatewayMessage::Resume {
                resume_token: token,
            },
        )
        .unwrap();
    assert_eq!(host.registry_entry().player_count, 1);
}

#[test]
fn test_live_session_complete() {
    let mut host = RuntimeHostService::start("arena-vanguard-live");
    let player = host.join("alice");
    host.submit_action(&player.player_id, PlayerAction::Move { dx: 6, dy: 0 })
        .unwrap();
    host.submit_action(
        &player.player_id,
        PlayerAction::Attack {
            enemy_id: "enemy-raider-1".to_owned(),
        },
    )
    .unwrap();
    let feed = host
        .submit_action(
            &player.player_id,
            PlayerAction::Interact {
                item_id: "field cache".to_owned(),
            },
        )
        .unwrap();
    let saved = host.disconnect(&player.player_id).unwrap();
    let restored = host.reconnect(&saved.resume_token).unwrap();
    assert_eq!(restored.character_id, saved.character_id);
    assert!(!feed.loot[0].available);
    assert!(host.status().replay_growth >= 5);
}
