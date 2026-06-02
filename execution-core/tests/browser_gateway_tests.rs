use execution_core::arena_vanguard_live::{
    BrowserGatewayEvent, BrowserGatewayMessage, BrowserWebSocketGateway, PlayerAction,
    RuntimeHostService,
};

fn joined_gateway() -> (RuntimeHostService, BrowserWebSocketGateway, String, String) {
    let mut host = RuntimeHostService::start("arena-vanguard-live");
    let mut gateway = BrowserWebSocketGateway::open("browser-a");
    let event = gateway
        .handle(
            &mut host,
            BrowserGatewayMessage::Join {
                player_seed: "alice".to_owned(),
            },
        )
        .expect("join event");
    match event {
        BrowserGatewayEvent::Joined { session, .. } => {
            let token = session.resume_token.clone();
            (host, gateway, session.player_id, token)
        }
        _ => panic!("expected joined event"),
    }
}

#[test]
fn test_websocket_join() {
    let (host, gateway, player_id, _) = joined_gateway();
    assert_eq!(gateway.deterministic_sequence, 1);
    assert_eq!(host.registry_entry().player_count, 1);
    assert_eq!(player_id, "player-alice");
    assert_eq!(host.status().websocket_connections, 1);
}

#[test]
fn test_websocket_resume() {
    let (mut host, mut gateway, player_id, token) = joined_gateway();
    gateway
        .handle(&mut host, BrowserGatewayMessage::Leave { player_id })
        .expect("leave event");
    let resumed = gateway
        .handle(
            &mut host,
            BrowserGatewayMessage::Resume {
                resume_token: token,
            },
        )
        .expect("resume event");
    assert!(matches!(resumed, BrowserGatewayEvent::Resumed { .. }));
    assert_eq!(host.registry_entry().player_count, 1);
}

#[test]
fn test_world_state_feed() {
    let (mut host, mut gateway, _, _) = joined_gateway();
    let event = gateway
        .handle(&mut host, BrowserGatewayMessage::SubscribeWorldState)
        .expect("feed event");
    match event {
        BrowserGatewayEvent::Feed(feed) => {
            assert_eq!(feed.session_id, "arena-vanguard-live");
            assert!(feed.zones.contains(&"Combat Area".to_owned()));
            assert!(!feed.loot.is_empty());
            assert!(feed.replay_size >= 2);
        }
        _ => panic!("expected feed"),
    }
}

#[test]
fn test_player_sync() {
    let (mut host, mut gateway, player_id, _) = joined_gateway();
    let event = gateway
        .handle(
            &mut host,
            BrowserGatewayMessage::Action {
                player_id: player_id.clone(),
                action: PlayerAction::Move { dx: 6, dy: 0 },
            },
        )
        .expect("movement feed");
    match event {
        BrowserGatewayEvent::Feed(feed) => assert_eq!(
            feed.players
                .iter()
                .find(|p| p.player_id == player_id)
                .unwrap()
                .position
                .zone,
            "Combat Area"
        ),
        _ => panic!("expected feed"),
    }
}

#[test]
fn test_enemy_sync() {
    let (mut host, mut gateway, player_id, _) = joined_gateway();
    let event = gateway
        .handle(
            &mut host,
            BrowserGatewayMessage::Action {
                player_id,
                action: PlayerAction::Attack {
                    enemy_id: "enemy-raider-1".to_owned(),
                },
            },
        )
        .expect("combat feed");
    match event {
        BrowserGatewayEvent::Feed(feed) => assert_eq!(feed.enemies[0].health, 75),
        _ => panic!("expected feed"),
    }
}

#[test]
fn test_loot_sync() {
    let (mut host, mut gateway, player_id, _) = joined_gateway();
    let event = gateway
        .handle(
            &mut host,
            BrowserGatewayMessage::Action {
                player_id,
                action: PlayerAction::Interact {
                    item_id: "field cache".to_owned(),
                },
            },
        )
        .expect("loot feed");
    match event {
        BrowserGatewayEvent::Feed(feed) => assert!(!feed.loot[0].available),
        _ => panic!("expected feed"),
    }
}

#[test]
fn test_disconnect_reconnect() {
    let (mut host, mut gateway, player_id, token) = joined_gateway();
    gateway
        .handle(&mut host, BrowserGatewayMessage::Leave { player_id })
        .expect("leave event");
    let before = host.registry_entry().replay_size;
    gateway
        .handle(
            &mut host,
            BrowserGatewayMessage::Resume {
                resume_token: token,
            },
        )
        .expect("resume event");
    assert!(host.registry_entry().replay_size > before);
    assert_eq!(host.registry_entry().player_count, 1);
}

#[test]
fn test_multiplayer_convergence() {
    let mut host = RuntimeHostService::start("arena-vanguard-live");
    let mut gateways = Vec::new();
    for index in 0..10 {
        let mut gateway = BrowserWebSocketGateway::open(format!("browser-{index}"));
        gateway.handle(
            &mut host,
            BrowserGatewayMessage::Join {
                player_seed: format!("p{index}"),
            },
        );
        gateways.push(gateway);
    }
    let feed = host.world_state_feed();
    assert_eq!(feed.players.len(), 10);
    assert_eq!(host.registry_entry().player_count, 10);
    assert_eq!(host.status().connected_browsers, 10);
}
