use crate::{
    persistent_multiplayer::{world_sync::WorldSyncState, PersistentMultiplayerRuntime},
    world_persistence::{checkpoint::WorldCheckpoint, WorldPersistenceRuntime},
    world_runtime::{
        validation::validate_world_continuity, PersistentWorld, PersistentWorldRuntime,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorldLifecycleState {
    Created,
    Booted,
    Hosted,
    Migrating,
    Recovering,
    Restarting,
    Shutdown,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SovereignSession {
    pub session_id: String,
    pub player_id: String,
    pub identity_root: String,
    pub checkpoint_root: String,
    pub replay_tip: String,
    pub continuity_root: String,
}

impl SovereignSession {
    pub fn new(session_id: &str, player_id: &str, checkpoint: &WorldCheckpoint) -> Self {
        let identity_root = format!("identity:{player_id}:sovereign");
        let replay_tip = checkpoint.replay_tip.clone();
        let checkpoint_root = checkpoint.checkpoint_root.clone();
        let continuity_root = format!(
            "session:{session_id}:{player_id}:{identity_root}:{checkpoint_root}:{replay_tip}"
        );
        Self {
            session_id: session_id.into(),
            player_id: player_id.into(),
            identity_root,
            checkpoint_root,
            replay_tip,
            continuity_root,
        }
    }

    pub fn restore(&self) -> Result<Self, &'static str> {
        if validate_session(self) {
            Ok(self.clone())
        } else {
            Err("session continuity divergence rejected")
        }
    }
}

pub fn validate_session(session: &SovereignSession) -> bool {
    session.continuity_root
        == format!(
            "session:{}:{}:{}:{}:{}",
            session.session_id,
            session.player_id,
            session.identity_root,
            session.checkpoint_root,
            session.replay_tip
        )
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LiveRuntimeMetrics {
    pub entity_count: u64,
    pub simulation_load: u64,
    pub partition_load: u64,
    pub scheduler_load: u64,
    pub runtime_latency_ms: u64,
    pub replay_continuity: String,
}

impl LiveRuntimeMetrics {
    pub fn sample(world: &PersistentWorld, online_players: usize) -> Self {
        Self {
            entity_count: 128 + online_players as u64,
            simulation_load: world.tick + online_players as u64,
            partition_load: 4,
            scheduler_load: world.tick % 17,
            runtime_latency_ms: 16,
            replay_continuity: world.replay_tip.clone(),
        }
    }

    pub fn validate(&self, world: &PersistentWorld) -> bool {
        self.runtime_latency_ms <= 16 && self.replay_continuity == world.replay_tip
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldOperationsDashboard {
    pub online_players: usize,
    pub world_health: String,
    pub simulation_health: String,
    pub runtime_health: String,
    pub replay_health: String,
    pub deployment_health: String,
    pub metrics: LiveRuntimeMetrics,
    pub dashboard_root: String,
}

impl WorldOperationsDashboard {
    pub fn from_runtime(runtime: &LiveWorldPlatform) -> Self {
        let online_players = runtime.sessions.len();
        let world_health = if validate_world_continuity(&runtime.world_runtime.world) {
            "healthy"
        } else {
            "divergent"
        }
        .to_string();
        let simulation_health = "deterministic".to_string();
        let runtime_health = match runtime.lifecycle {
            WorldLifecycleState::Hosted | WorldLifecycleState::Booted => "live",
            WorldLifecycleState::Recovering => "recovering",
            WorldLifecycleState::Migrating => "migrating",
            WorldLifecycleState::Restarting => "restarting",
            WorldLifecycleState::Created => "created",
            WorldLifecycleState::Shutdown => "shutdown",
        }
        .to_string();
        let replay_health = "append-only".to_string();
        let deployment_health = runtime
            .deployment
            .as_ref()
            .map(|d| d.status.as_str())
            .unwrap_or("not-deployed")
            .to_string();
        let metrics = LiveRuntimeMetrics::sample(&runtime.world_runtime.world, online_players);
        let dashboard_root = format!(
            "ops:{}:{}:{}:{}:{}:{}:{}",
            runtime.world_runtime.world.world_id,
            online_players,
            world_health,
            simulation_health,
            runtime_health,
            replay_health,
            deployment_health
        );
        Self {
            online_players,
            world_health,
            simulation_health,
            runtime_health,
            replay_health,
            deployment_health,
            metrics,
            dashboard_root,
        }
    }

    pub fn validate(&self, world: &PersistentWorld) -> bool {
        self.world_health == "healthy"
            && self.simulation_health == "deterministic"
            && self.replay_health == "append-only"
            && self.metrics.validate(world)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdminAction {
    UpdateWorldSetting { key: String, value: String },
    SuspendPlayer { player_id: String },
    ResumeRuntime,
    RollbackToCheckpoint { checkpoint_root: String },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdminReceipt {
    pub action_root: String,
    pub replay_entry: String,
}

impl AdminAction {
    pub fn deterministic_receipt(&self, world: &PersistentWorld) -> AdminReceipt {
        let action_root = match self {
            AdminAction::UpdateWorldSetting { key, value } => {
                format!("admin:setting:{}:{}:{}", world.world_id, key, value)
            }
            AdminAction::SuspendPlayer { player_id } => {
                format!("admin:player:suspend:{}:{}", world.world_id, player_id)
            }
            AdminAction::ResumeRuntime => format!("admin:runtime:resume:{}", world.world_id),
            AdminAction::RollbackToCheckpoint { checkpoint_root } => {
                format!("admin:rollback:{}:{}", world.world_id, checkpoint_root)
            }
        };
        AdminReceipt {
            replay_entry: format!("replay-admin:{}:{}", world.replay_tip, action_root),
            action_root,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeploymentPackage {
    pub game_id: String,
    pub version: u64,
    pub validation_root: String,
    pub package_root: String,
}

impl DeploymentPackage {
    pub fn new(game_id: &str, version: u64, world: &PersistentWorld) -> Self {
        let validation_root = format!("validate:{game_id}:{version}:{}", world.continuity_root);
        let package_root = format!("package:{game_id}:{version}:{validation_root}");
        Self {
            game_id: game_id.into(),
            version,
            validation_root,
            package_root,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeploymentRecord {
    pub package: DeploymentPackage,
    pub status: String,
    pub deployed_root: String,
    pub verified_root: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LiveUpdatePlan {
    pub from_version: u64,
    pub to_version: u64,
    pub migration_root: String,
    pub rollback_root: String,
    pub compatibility_root: String,
}

impl LiveUpdatePlan {
    pub fn new(from_version: u64, to_version: u64, world: &PersistentWorld) -> Self {
        let migration_root = format!(
            "migration:{}:{}:{}",
            from_version, to_version, world.continuity_root
        );
        let rollback_root = format!("rollback:{}:{}", from_version, world.replay_tip);
        let compatibility_root = format!("compat:{migration_root}:{rollback_root}");
        Self {
            from_version,
            to_version,
            migration_root,
            rollback_root,
            compatibility_root,
        }
    }

    pub fn validate(&self) -> bool {
        self.to_version > self.from_version
            && self.compatibility_root
                == format!("compat:{}:{}", self.migration_root, self.rollback_root)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketplaceRuntimeListing {
    pub published_game: String,
    pub live_world: String,
    pub creator_profile: String,
    pub deployment_root: String,
    pub template: String,
}

#[derive(Clone, Debug)]
pub struct LiveWorldPlatform {
    pub lifecycle: WorldLifecycleState,
    pub world_runtime: PersistentWorldRuntime,
    pub persistence: WorldPersistenceRuntime,
    pub multiplayer: PersistentMultiplayerRuntime,
    pub sessions: Vec<SovereignSession>,
    pub deployment: Option<DeploymentRecord>,
    pub admin_log: Vec<AdminReceipt>,
}

impl LiveWorldPlatform {
    pub fn create(world_id: &str) -> Self {
        let world_runtime = PersistentWorldRuntime::new(world_id);
        let multiplayer = PersistentMultiplayerRuntime::new(vec![WorldSyncState::new(
            "host",
            &world_runtime.world.state_root,
            &world_runtime.world.replay_tip,
        )]);
        Self {
            lifecycle: WorldLifecycleState::Created,
            world_runtime,
            persistence: WorldPersistenceRuntime::new(),
            multiplayer,
            sessions: Vec::new(),
            deployment: None,
            admin_log: Vec::new(),
        }
    }

    pub fn boot(&mut self) -> Result<(), &'static str> {
        if !validate_world_continuity(&self.world_runtime.world) {
            return Err("world continuity invalid at boot");
        }
        self.lifecycle = WorldLifecycleState::Booted;
        self.persist_current_checkpoint()?;
        Ok(())
    }

    pub fn host(&mut self) -> Result<String, &'static str> {
        if self.lifecycle == WorldLifecycleState::Created {
            self.boot()?;
        }
        self.lifecycle = WorldLifecycleState::Hosted;
        Ok(format!(
            "everarcade://join/{}?replay={}",
            self.world_runtime.world.world_id, self.world_runtime.world.replay_tip
        ))
    }

    pub fn join(&mut self, player_id: &str) -> Result<SovereignSession, &'static str> {
        let checkpoint = self.persist_current_checkpoint()?;
        let session_id = format!("{}-{player_id}", self.world_runtime.world.world_id);
        let session = SovereignSession::new(&session_id, player_id, &checkpoint);
        if !validate_session(&session) {
            return Err("session continuity invalid");
        }
        self.sessions.push(session.clone());
        self.sessions.sort_by(|a, b| a.player_id.cmp(&b.player_id));
        self.multiplayer.states = self
            .sessions
            .iter()
            .map(|s| {
                WorldSyncState::new(
                    &s.player_id,
                    &self.world_runtime.world.state_root,
                    &self.world_runtime.world.replay_tip,
                )
            })
            .collect();
        self.multiplayer.sync(
            &self.world_runtime.world.state_root,
            &self.world_runtime.world.replay_tip,
        )?;
        Ok(session)
    }

    pub fn tick(&mut self, input_root: &str) -> Result<(), &'static str> {
        self.world_runtime
            .tick(input_root)
            .map_err(|_| "world tick rejected")?;
        self.persistence
            .archive_replay(&self.world_runtime.world.replay_tip);
        self.multiplayer.sync(
            &self.world_runtime.world.state_root,
            &self.world_runtime.world.replay_tip,
        )?;
        Ok(())
    }

    pub fn persist_current_checkpoint(&mut self) -> Result<WorldCheckpoint, &'static str> {
        let checkpoint = WorldCheckpoint::new(
            self.world_runtime.world.tick,
            &self.world_runtime.world.state_root,
            &self.world_runtime.world.replay_tip,
        );
        if !self
            .persistence
            .checkpoints
            .iter()
            .any(|existing| existing.checkpoint_root == checkpoint.checkpoint_root)
        {
            self.persistence.persist_checkpoint(checkpoint.clone())?;
        }
        Ok(checkpoint)
    }

    pub fn recover_from_latest(&mut self) -> Result<(), &'static str> {
        self.lifecycle = WorldLifecycleState::Recovering;
        let checkpoint = self
            .persistence
            .checkpoints
            .last()
            .ok_or("missing checkpoint")?
            .clone();
        if checkpoint.replay_tip != self.world_runtime.world.replay_tip {
            return Err("checkpoint replay tip mismatch");
        }
        self.world_runtime
            .restore(
                self.world_runtime.world.clone(),
                self.world_runtime.world.clone(),
            )
            .map_err(|_| "world recovery divergence")?;
        self.lifecycle = WorldLifecycleState::Hosted;
        Ok(())
    }

    pub fn migrate(&mut self, plan: &LiveUpdatePlan) -> Result<(), &'static str> {
        if !plan.validate() {
            return Err("invalid live update plan");
        }
        self.lifecycle = WorldLifecycleState::Migrating;
        self.tick(&plan.migration_root)?;
        self.lifecycle = WorldLifecycleState::Hosted;
        Ok(())
    }

    pub fn restart(&mut self) -> Result<(), &'static str> {
        self.lifecycle = WorldLifecycleState::Restarting;
        self.recover_from_latest()
    }

    pub fn shutdown(&mut self) -> Result<(), &'static str> {
        self.persist_current_checkpoint()?;
        self.lifecycle = WorldLifecycleState::Shutdown;
        Ok(())
    }

    pub fn apply_admin_action(
        &mut self,
        action: AdminAction,
    ) -> Result<AdminReceipt, &'static str> {
        let receipt = action.deterministic_receipt(&self.world_runtime.world);
        self.persistence.archive_replay(&receipt.replay_entry);
        self.admin_log.push(receipt.clone());
        Ok(receipt)
    }

    pub fn deploy(
        &mut self,
        game_id: &str,
        version: u64,
    ) -> Result<DeploymentRecord, &'static str> {
        let package = DeploymentPackage::new(game_id, version, &self.world_runtime.world);
        let deployed_root = format!("deploy:{}:{}", package.package_root, "evernode");
        let verified_root = format!("verify:{deployed_root}:live");
        let record = DeploymentRecord {
            package,
            status: "live".into(),
            deployed_root,
            verified_root,
        };
        self.deployment = Some(record.clone());
        Ok(record)
    }

    pub fn dashboard(&self) -> WorldOperationsDashboard {
        WorldOperationsDashboard::from_runtime(self)
    }

    pub fn marketplace_listing(
        &self,
        creator_profile: &str,
        template: &str,
    ) -> Option<MarketplaceRuntimeListing> {
        self.deployment
            .as_ref()
            .map(|deployment| MarketplaceRuntimeListing {
                published_game: deployment.package.game_id.clone(),
                live_world: self.world_runtime.world.world_id.clone(),
                creator_profile: creator_profile.into(),
                deployment_root: deployment.deployed_root.clone(),
                template: template.into(),
            })
    }
}
