use super::disconnect;
use super::heartbeat::{apply_timeout, record_heartbeat, DEFAULT_TIMEOUT_TICKS};
use super::player::{Character, CharacterId, PlayerId, SessionId};
use super::presence::{Presence, PresenceState};
use super::reconnect;
use super::spawn;
use contract_api::protocol_records::ProtocolRecord;
use rustrigs::combat::{apply_damage, apply_healing, CombatInput};
use rustrigs::inventory::{add_item, InventoryInput};
use rustrigs::movement::{move_actor, MovementInput};
use rustrigs::progression::{grant_experience, unlock_milestone, ProgressionInput};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnemyKind {
    Goblin,
    Skeleton,
    Bandit,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Enemy {
    pub id: String,
    pub kind: EnemyKind,
    pub health: i64,
    pub attack: i64,
    pub x: i64,
    pub y: i64,
    pub alive: bool,
    pub respawn_tick: Option<u64>,
}

impl Enemy {
    pub fn new(kind: EnemyKind, index: u64, x: i64, y: i64) -> Self {
        let (health, attack) = match kind {
            EnemyKind::Goblin => (30, 6),
            EnemyKind::Skeleton => (45, 8),
            EnemyKind::Bandit => (60, 10),
        };
        Self {
            id: format!("enemy-{kind:?}-{index}"),
            kind,
            health,
            attack,
            x,
            y,
            alive: true,
            respawn_tick: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LootDrop {
    pub id: String,
    pub item: String,
    pub quantity: u64,
    pub x: i64,
    pub y: i64,
    pub collected: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArenaTelemetry {
    pub session_count: usize,
    pub player_count: usize,
    pub runtime_tick: u64,
    pub replay_growth: usize,
    pub checkpoint_age: u64,
    pub runtime_health: String,
    pub world_health: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArenaVanguardRuntime {
    pub game_id: String,
    pub tick: u64,
    pub presences: BTreeMap<PlayerId, Presence>,
    pub characters: BTreeMap<CharacterId, Character>,
    pub enemies: BTreeMap<String, Enemy>,
    pub loot: BTreeMap<String, LootDrop>,
    pub replay: Vec<String>,
    pub checkpoint: Option<Vec<u8>>,
    pub last_checkpoint_tick: u64,
}

impl Default for ArenaVanguardRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl ArenaVanguardRuntime {
    pub fn new() -> Self {
        let mut enemies = BTreeMap::new();
        for enemy in [
            Enemy::new(EnemyKind::Goblin, 1, 5, 0),
            Enemy::new(EnemyKind::Skeleton, 1, 7, 0),
            Enemy::new(EnemyKind::Bandit, 1, 9, 0),
        ] {
            enemies.insert(enemy.id.clone(), enemy);
        }
        let mut loot = BTreeMap::new();
        loot.insert(
            "loot-gold-1".to_owned(),
            LootDrop {
                id: "loot-gold-1".to_owned(),
                item: "gold".to_owned(),
                quantity: 25,
                x: 2,
                y: 0,
                collected: false,
            },
        );
        loot.insert(
            "loot-potion-1".to_owned(),
            LootDrop {
                id: "loot-potion-1".to_owned(),
                item: "health potion".to_owned(),
                quantity: 1,
                x: 3,
                y: 0,
                collected: false,
            },
        );
        loot.insert(
            "loot-sword-1".to_owned(),
            LootDrop {
                id: "loot-sword-1".to_owned(),
                item: "iron sword".to_owned(),
                quantity: 1,
                x: 4,
                y: 0,
                collected: false,
            },
        );
        Self {
            game_id: "arena-vanguard".to_owned(),
            tick: 0,
            presences: BTreeMap::new(),
            characters: BTreeMap::new(),
            enemies,
            loot,
            replay: Vec::new(),
            checkpoint: None,
            last_checkpoint_tick: 0,
        }
    }

    fn advance(&mut self, action: &str) {
        self.tick += 1;
        self.replay.push(format!("{}:{action}", self.tick));
    }

    pub fn join(&mut self, seed: &str) -> (SessionId, CharacterId) {
        self.advance("join");
        let player_id = PlayerId::new(seed);
        let session_id = SessionId::new(&self.game_id, &player_id);
        self.presences.insert(
            player_id.clone(),
            Presence::online(player_id.clone(), session_id.clone(), self.tick),
        );
        let mut character = self
            .characters
            .remove(&CharacterId::for_player(&player_id))
            .unwrap_or_else(|| Character::starter_warrior(player_id));
        spawn::spawn_player(&mut character, self.tick);
        let character_id = character.id.clone();
        self.characters.insert(character_id.clone(), character);
        (session_id, character_id)
    }

    pub fn leave(&mut self, player_id: &PlayerId) {
        self.advance("leave");
        if let Some(presence) = self.presences.get_mut(player_id) {
            disconnect::leave(presence, self.tick);
        }
    }

    pub fn disconnect(&mut self, player_id: &PlayerId) {
        self.advance("disconnect");
        if let Some(presence) = self.presences.get_mut(player_id) {
            disconnect::disconnect(presence, self.tick);
        }
        self.save_checkpoint();
    }

    pub fn reconnect(&mut self, player_id: &PlayerId) -> Option<CharacterId> {
        self.advance("reconnect");
        self.presences.get_mut(player_id).map(|presence| {
            reconnect::reconnect(presence, self.tick);
            CharacterId::for_player(player_id)
        })
    }

    pub fn heartbeat(&mut self, player_id: &PlayerId) {
        self.advance("heartbeat");
        if let Some(presence) = self.presences.get_mut(player_id) {
            record_heartbeat(presence, self.tick);
        }
    }

    pub fn timeout_idle(&mut self) -> usize {
        self.advance("timeout");
        let mut timed_out = 0;
        for presence in self.presences.values_mut() {
            if apply_timeout(presence, self.tick, DEFAULT_TIMEOUT_TICKS) {
                timed_out += 1;
            }
        }
        timed_out
    }

    pub fn move_player(
        &mut self,
        character_id: &CharacterId,
        dx: i64,
        dy: i64,
    ) -> Vec<ProtocolRecord> {
        self.advance("move");
        let character = self
            .characters
            .get_mut(character_id)
            .expect("character must exist");
        character.x = (character.x + dx).clamp(-100, 100);
        character.y = (character.y + dy).clamp(-100, 100);
        move_actor(MovementInput {
            actor: character.id.0.clone(),
            world: "Arena Outpost".to_owned(),
            x: character.x,
            y: character.y,
            min: -100,
            max: 100,
            tick: self.tick,
        })
    }

    pub fn attack_enemy(
        &mut self,
        character_id: &CharacterId,
        enemy_id: &str,
    ) -> Vec<ProtocolRecord> {
        self.advance("attack");
        let character = self
            .characters
            .get(character_id)
            .expect("character must exist")
            .clone();
        let enemy = self.enemies.get_mut(enemy_id).expect("enemy must exist");
        if !enemy.alive {
            return Vec::new();
        }
        enemy.health -= character.attack;
        let mut records = apply_damage(CombatInput {
            actor: character.id.0.clone(),
            target: enemy.id.clone(),
            amount: character.attack as u64,
            status: "attack".to_owned(),
            tick: self.tick,
        });
        let defeated = if enemy.health <= 0 {
            enemy.alive = false;
            enemy.respawn_tick = Some(self.tick + 30);
            Some((enemy.id.clone(), enemy.x, enemy.y))
        } else {
            None
        };
        if let Some((defeated_id, x, y)) = defeated {
            records.extend(self.grant_xp(character_id, 100));
            let drop_id = format!("loot-{defeated_id}-{}", self.tick);
            self.loot.insert(
                drop_id.clone(),
                LootDrop {
                    id: drop_id,
                    item: "gold".to_owned(),
                    quantity: 10,
                    x,
                    y,
                    collected: false,
                },
            );
        }
        records
    }

    pub fn heal_player(&mut self, character_id: &CharacterId, amount: u64) -> Vec<ProtocolRecord> {
        self.advance("heal");
        let character = self
            .characters
            .get_mut(character_id)
            .expect("character must exist");
        character.health = (character.health + amount as i64).min(character.max_health);
        apply_healing(CombatInput {
            actor: character.id.0.clone(),
            target: character.id.0.clone(),
            amount,
            status: "heal".to_owned(),
            tick: self.tick,
        })
    }

    pub fn collect_loot(
        &mut self,
        character_id: &CharacterId,
        loot_id: &str,
    ) -> Vec<ProtocolRecord> {
        self.advance("loot");
        let character = self
            .characters
            .get_mut(character_id)
            .expect("character must exist");
        let drop = self.loot.get_mut(loot_id).expect("loot must exist");
        if drop.collected {
            return Vec::new();
        }
        drop.collected = true;
        character.add_item(&drop.item, drop.quantity);
        add_item(InventoryInput {
            owner: character.id.0.clone(),
            item: drop.item.clone(),
            quantity: drop.quantity,
            counterparty: "world".to_owned(),
            slot: "bag".to_owned(),
            tick: self.tick,
        })
    }

    pub fn grant_xp(&mut self, character_id: &CharacterId, amount: u64) -> Vec<ProtocolRecord> {
        let character = self
            .characters
            .get_mut(character_id)
            .expect("character must exist");
        character.experience += amount;
        let mut records = grant_experience(ProgressionInput {
            actor: character.id.0.clone(),
            track: "combat".to_owned(),
            amount,
            milestone: "xp-gain".to_owned(),
            tick: self.tick,
        });
        while character.experience >= character.level * 100 {
            character.experience -= character.level * 100;
            character.level += 1;
            character.max_health += 10;
            character.attack += 2;
            character.defense += 1;
            records.extend(unlock_milestone(ProgressionInput {
                actor: character.id.0.clone(),
                track: "level".to_owned(),
                amount: character.level,
                milestone: format!("level-{}", character.level),
                tick: self.tick,
            }));
        }
        records
    }

    pub fn run_enemy_ai_tick(&mut self) {
        self.advance("enemy-ai");
        for enemy in self.enemies.values_mut() {
            if !enemy.alive {
                if enemy
                    .respawn_tick
                    .is_some_and(|respawn| self.tick >= respawn)
                {
                    enemy.health = match enemy.kind {
                        EnemyKind::Goblin => 30,
                        EnemyKind::Skeleton => 45,
                        EnemyKind::Bandit => 60,
                    };
                    enemy.alive = true;
                    enemy.respawn_tick = None;
                }
                continue;
            }
            enemy.x = match self.tick % 3 {
                0 => enemy.x - 1,
                1 => enemy.x + 1,
                _ => enemy.x,
            };
        }
    }

    pub fn respawn_player(&mut self, character_id: &CharacterId) -> Vec<ProtocolRecord> {
        self.advance("respawn");
        let character = self
            .characters
            .get_mut(character_id)
            .expect("character must exist");
        spawn::respawn_player(character, self.tick)
    }

    pub fn save_checkpoint(&mut self) {
        self.checkpoint =
            Some(serde_json::to_vec(self).expect("runtime checkpoint must serialize"));
        self.last_checkpoint_tick = self.tick;
    }

    pub fn restore_checkpoint(&self) -> Self {
        serde_json::from_slice(self.checkpoint.as_ref().expect("checkpoint must exist"))
            .expect("runtime checkpoint must restore")
    }

    pub fn replay_equivalent(&self, other: &Self) -> bool {
        self.characters == other.characters
            && self.presences == other.presences
            && self.loot == other.loot
    }

    pub fn telemetry(&self) -> ArenaTelemetry {
        ArenaTelemetry {
            session_count: self.presences.len(),
            player_count: self
                .presences
                .values()
                .filter(|p| matches!(p.state, PresenceState::Online))
                .count(),
            runtime_tick: self.tick,
            replay_growth: self.replay.len(),
            checkpoint_age: self.tick.saturating_sub(self.last_checkpoint_tick),
            runtime_health: "healthy".to_owned(),
            world_health: "healthy".to_owned(),
        }
    }
}
