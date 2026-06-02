use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuntimeHealth {
    Healthy,
    Checkpointing,
    Recovering,
    Failed,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum GatewayHealth {
    Healthy,
    Warning,
    Failed,
    Recovering,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlayerAction {
    Move { dx: i32, dy: i32 },
    Attack { enemy_id: String },
    Interact { item_id: String },
    UseItem { item_id: String },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub zone: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerSessionState {
    pub player_id: String,
    pub character_id: String,
    pub position: Position,
    pub health: u32,
    pub energy: u32,
    pub inventory: Vec<String>,
    pub xp: u32,
    pub level: u32,
    pub connected: bool,
    pub resume_token: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnemyState {
    pub enemy_id: String,
    pub position: Position,
    pub health: u32,
    pub status: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldStateFeed {
    pub tick: u64,
    pub session_id: String,
    pub players: Vec<PlayerSessionState>,
    pub enemies: Vec<EnemyState>,
    pub world_zones: Vec<String>,
    pub replay_size: u64,
    pub checkpoint_age: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionRegistryEntry {
    pub session_id: String,
    pub player_count: usize,
    pub runtime_health: RuntimeHealth,
    pub checkpoint_age: u64,
    pub replay_size: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct LiveSessionMetrics {
    pub join_rate: u64,
    pub reconnect_rate: u64,
    pub action_throughput: u64,
    pub gateway_latency_ms: u64,
    pub session_duration_ticks: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperatorStatus {
    pub active_sessions: usize,
    pub players_online: usize,
    pub gateway_status: GatewayHealth,
    pub runtime_status: RuntimeHealth,
    pub replay_growth: u64,
    pub checkpoint_age: u64,
    pub recovery_state: String,
    pub metrics: LiveSessionMetrics,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuntimeHostService {
    pub session_id: String,
    pub runtime_health: RuntimeHealth,
    tick: u64,
    replay: Vec<String>,
    checkpoints: Vec<String>,
    player_sessions: BTreeMap<String, PlayerSessionState>,
    enemies: BTreeMap<String, EnemyState>,
    metrics: LiveSessionMetrics,
}

impl RuntimeHostService {
    pub fn start(session_id: impl Into<String>) -> Self {
        let session_id = session_id.into();
        let mut enemies = BTreeMap::new();
        enemies.insert(
            "enemy-raider-1".to_owned(),
            EnemyState {
                enemy_id: "enemy-raider-1".to_owned(),
                position: Position {
                    x: 8,
                    y: 0,
                    zone: "Combat Area".to_owned(),
                },
                health: 100,
                status: "hostile".to_owned(),
            },
        );
        Self {
            session_id,
            runtime_health: RuntimeHealth::Healthy,
            tick: 0,
            replay: Vec::new(),
            checkpoints: Vec::new(),
            player_sessions: BTreeMap::new(),
            enemies,
            metrics: LiveSessionMetrics {
                gateway_latency_ms: 1,
                ..LiveSessionMetrics::default()
            },
        }
    }

    pub fn stop(&mut self) {
        self.runtime_health = RuntimeHealth::Failed;
    }

    pub fn restart(&mut self) {
        self.runtime_health = RuntimeHealth::Healthy;
    }

    pub fn recover(&mut self) {
        self.runtime_health = RuntimeHealth::Recovering;
        self.record_runtime_owned("recover");
        self.runtime_health = RuntimeHealth::Healthy;
    }

    pub fn status(&self) -> OperatorStatus {
        OperatorStatus {
            active_sessions: 1,
            players_online: self
                .player_sessions
                .values()
                .filter(|p| p.connected)
                .count(),
            gateway_status: GatewayHealth::Healthy,
            runtime_status: self.runtime_health.clone(),
            replay_growth: self.replay.len() as u64,
            checkpoint_age: self.checkpoint_age(),
            recovery_state: if self.runtime_health == RuntimeHealth::Recovering {
                "recovering"
            } else {
                "stable"
            }
            .to_owned(),
            metrics: self.metrics.clone(),
        }
    }

    pub fn registry_entry(&self) -> SessionRegistryEntry {
        SessionRegistryEntry {
            session_id: self.session_id.clone(),
            player_count: self
                .player_sessions
                .values()
                .filter(|p| p.connected)
                .count(),
            runtime_health: self.runtime_health.clone(),
            checkpoint_age: self.checkpoint_age(),
            replay_size: self.replay.len() as u64,
        }
    }

    pub fn join(&mut self, player_seed: &str) -> PlayerSessionState {
        self.tick += 1;
        self.metrics.join_rate += 1;
        let player_id = format!("player-{player_seed}");
        let mut session = self
            .player_sessions
            .get(&player_id)
            .cloned()
            .unwrap_or_else(|| {
                let resume_token = token_for(&self.session_id, &player_id);
                PlayerSessionState {
                    player_id: player_id.clone(),
                    character_id: format!("character-{player_seed}"),
                    position: Position {
                        x: 0,
                        y: 0,
                        zone: "Spawn Area".to_owned(),
                    },
                    health: 100,
                    energy: 50,
                    inventory: vec!["starter blade".to_owned()],
                    xp: 0,
                    level: 1,
                    connected: false,
                    resume_token,
                }
            });
        session.connected = true;
        self.player_sessions.insert(player_id, session.clone());
        self.record_runtime_owned("join");
        session
    }

    pub fn disconnect(&mut self, player_id: &str) -> Option<PlayerSessionState> {
        self.tick += 1;
        let mut session = self.player_sessions.get(player_id).cloned()?;
        session.connected = false;
        self.player_sessions
            .insert(player_id.to_owned(), session.clone());
        self.persist_checkpoint();
        self.record_runtime_owned("disconnect");
        Some(session)
    }

    pub fn reconnect(&mut self, resume_token: &str) -> Option<PlayerSessionState> {
        self.tick += 1;
        self.metrics.reconnect_rate += 1;
        let player_id = self
            .player_sessions
            .iter()
            .find(|(_, session)| session.resume_token == resume_token)
            .map(|(id, _)| id.clone())?;
        let mut session = self.player_sessions.get(&player_id).cloned()?;
        session.connected = true;
        self.player_sessions.insert(player_id, session.clone());
        self.record_runtime_owned("reconnect");
        Some(session)
    }

    pub fn submit_action(
        &mut self,
        player_id: &str,
        action: PlayerAction,
    ) -> Option<WorldStateFeed> {
        self.tick += 1;
        self.metrics.action_throughput += 1;
        let mut player = self.player_sessions.get(player_id).cloned()?;
        match action {
            PlayerAction::Move { dx, dy } => {
                player.position.x += dx;
                player.position.y += dy;
                player.position.zone = zone_for(player.position.x, player.position.y);
                player.energy = player.energy.saturating_sub(1);
            }
            PlayerAction::Attack { enemy_id } => {
                if let Some(enemy) = self.enemies.get_mut(&enemy_id) {
                    enemy.health = enemy.health.saturating_sub(25);
                    enemy.status = if enemy.health == 0 {
                        "defeated"
                    } else {
                        "hostile"
                    }
                    .to_owned();
                    player.xp += 35;
                    if player.xp >= player.level * 100 {
                        player.xp -= player.level * 100;
                        player.level += 1;
                    }
                }
            }
            PlayerAction::Interact { item_id } => {
                player.inventory.push(item_id);
                player.xp += 10;
            }
            PlayerAction::UseItem { item_id } => {
                if let Some(index) = player.inventory.iter().position(|owned| owned == &item_id) {
                    player.inventory.remove(index);
                    player.health = (player.health + 20).min(100);
                }
            }
        }
        self.player_sessions.insert(player_id.to_owned(), player);
        self.record_runtime_owned("action");
        Some(self.world_state_feed())
    }

    pub fn world_state_feed(&self) -> WorldStateFeed {
        WorldStateFeed {
            tick: self.tick,
            session_id: self.session_id.clone(),
            players: self.player_sessions.values().cloned().collect(),
            enemies: self.enemies.values().cloned().collect(),
            world_zones: vec![
                "Spawn Area".to_owned(),
                "Combat Area".to_owned(),
                "Loot Area".to_owned(),
                "Safe Area".to_owned(),
            ],
            replay_size: self.replay.len() as u64,
            checkpoint_age: self.checkpoint_age(),
        }
    }

    pub fn replay_equivalence_root(&self) -> String {
        let mut h = Sha256::new();
        h.update(b"everarcade:arena-vanguard-live-session:v1");
        for record in &self.replay {
            h.update(record.as_bytes());
        }
        format!("sha256:{}", hex::encode(h.finalize()))
    }

    fn persist_checkpoint(&mut self) {
        self.runtime_health = RuntimeHealth::Checkpointing;
        self.checkpoints.push(self.replay_equivalence_root());
        self.runtime_health = RuntimeHealth::Healthy;
    }

    fn record_runtime_owned(&mut self, kind: &str) {
        self.replay
            .push(format!("{}:{}:{}", self.tick, self.session_id, kind));
        self.metrics.session_duration_ticks = self.tick;
    }

    fn checkpoint_age(&self) -> u64 {
        self.replay.len().saturating_sub(self.checkpoints.len()) as u64
    }
}

#[derive(Clone, Debug)]
pub struct ArenaVanguardGateway {
    pub health: GatewayHealth,
    pub attached_session_id: Option<String>,
}

impl ArenaVanguardGateway {
    pub fn launch_and_attach(host: &RuntimeHostService) -> Self {
        Self {
            health: GatewayHealth::Healthy,
            attached_session_id: Some(host.session_id.clone()),
        }
    }

    pub fn health_check(&mut self, host: &RuntimeHostService) -> GatewayHealth {
        self.health = if host.runtime_health == RuntimeHealth::Failed {
            GatewayHealth::Recovering
        } else {
            GatewayHealth::Healthy
        };
        self.health.clone()
    }

    pub fn discover_runtime<'a>(&self, host: &'a RuntimeHostService) -> Option<&'a str> {
        self.attached_session_id
            .as_deref()
            .filter(|id| *id == host.session_id)
            .map(|_| host.session_id.as_str())
    }

    pub fn reattach(&mut self, host: &RuntimeHostService) {
        self.attached_session_id = Some(host.session_id.clone());
        self.health = GatewayHealth::Healthy;
    }
}

pub fn validate_multiplayer(player_count: usize) -> bool {
    let mut host = RuntimeHostService::start("arena-vanguard-live");
    for index in 0..player_count {
        let player = host.join(&format!("p{index}"));
        let _ = host.submit_action(
            &player.player_id,
            PlayerAction::Move {
                dx: index as i32,
                dy: 1,
            },
        );
        let _ = host.submit_action(
            &player.player_id,
            PlayerAction::Attack {
                enemy_id: "enemy-raider-1".to_owned(),
            },
        );
        let _ = host.submit_action(
            &player.player_id,
            PlayerAction::Interact {
                item_id: "field cache".to_owned(),
            },
        );
        let saved = host
            .disconnect(&player.player_id)
            .expect("player disconnect should persist");
        let restored = host
            .reconnect(&saved.resume_token)
            .expect("resume token should restore");
        if restored.character_id != saved.character_id
            || restored.inventory != saved.inventory
            || restored.level != saved.level
        {
            return false;
        }
    }
    host.registry_entry().player_count == player_count
}

fn token_for(session_id: &str, player_id: &str) -> String {
    let mut h = Sha256::new();
    h.update(session_id.as_bytes());
    h.update(player_id.as_bytes());
    format!("resume:{}", hex::encode(h.finalize()))
}

fn zone_for(x: i32, y: i32) -> String {
    if x == 0 && y == 0 {
        "Spawn Area".to_owned()
    } else if x.abs() <= 2 && y.abs() <= 2 {
        "Safe Area".to_owned()
    } else if x >= 5 {
        "Combat Area".to_owned()
    } else {
        "Loot Area".to_owned()
    }
}
