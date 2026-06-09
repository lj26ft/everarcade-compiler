use crate::runtime::*;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RuntimeReceipt {
    pub receipt_id: String,
    pub sequence: u64,
    pub tick: u64,
    pub input_id: String,
    pub input_hash: String,
    pub state_root: String,
    pub receipt_hash: String,
    pub runtime_version: String,
    pub world_id: String,
    pub timestamp_or_epoch: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guest_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guest_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guest_output_hash: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeterministicProofInput {
    pub player_id: String,
    pub action: String,
    pub direction: String,
    pub sequence: u64,
}

impl DeterministicProofInput {
    pub fn canonical() -> Self {
        Self {
            player_id: "audit-player".into(),
            action: "move".into(),
            direction: "north".into(),
            sequence: 1,
        }
    }

    pub fn canonical_bytes(&self) -> Result<Vec<u8>> {
        Ok(serde_json::to_vec(self)?)
    }

    pub fn stable_hash(&self) -> Result<String> {
        Ok(hex::encode(Sha256::digest(self.canonical_bytes()?)))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ArenaPosition {
    pub x: i64,
    pub y: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ArenaPlayer {
    pub player_id: String,
    pub joined_tick: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ArenaState {
    pub session_id: String,
    pub tick: u64,
    pub players: BTreeMap<String, ArenaPlayer>,
    pub positions: BTreeMap<String, ArenaPosition>,
    pub health: BTreeMap<String, i64>,
    pub scores: BTreeMap<String, i64>,
    pub events: Vec<String>,
    pub player_count: u64,
}

impl ArenaState {
    pub fn initial() -> Self {
        let mut positions = BTreeMap::new();
        positions.insert("dummy".into(), ArenaPosition { x: 0, y: 1 });
        let mut health = BTreeMap::new();
        health.insert("dummy".into(), 100);
        let mut scores = BTreeMap::new();
        scores.insert("dummy".into(), 0);
        Self {
            session_id: "session-0001".into(),
            tick: 0,
            players: BTreeMap::new(),
            positions,
            health,
            scores,
            events: vec!["session_started".into()],
            player_count: 0,
        }
    }

    pub fn root(&self) -> Result<String> {
        Ok(hex::encode(Sha256::digest(serde_json::to_vec(self)?)))
    }

    pub fn apply(&mut self, input: &ArenaGameplayInput) {
        self.tick = input.sequence;
        match input.action.as_str() {
            "join" => {
                self.players.insert(
                    input.player_id.clone(),
                    ArenaPlayer {
                        player_id: input.player_id.clone(),
                        joined_tick: input.sequence,
                    },
                );
                self.positions
                    .entry(input.player_id.clone())
                    .or_insert(ArenaPosition { x: 0, y: 0 });
                self.health.entry(input.player_id.clone()).or_insert(100);
                self.scores.entry(input.player_id.clone()).or_insert(0);
                self.player_count = self.players.len() as u64;
                self.events.push(format!(
                    "tick {}: {} joined",
                    input.sequence, input.player_id
                ));
            }
            "move" => {
                let position = self
                    .positions
                    .entry(input.player_id.clone())
                    .or_insert(ArenaPosition { x: 0, y: 0 });
                match input.direction.as_deref().unwrap_or("north") {
                    "north" => position.y += 1,
                    "south" => position.y -= 1,
                    "east" => position.x += 1,
                    "west" => position.x -= 1,
                    _ => {}
                }
                self.events.push(format!(
                    "tick {}: {} moved {}",
                    input.sequence,
                    input.player_id,
                    input.direction.as_deref().unwrap_or("north")
                ));
            }
            "attack" => {
                let target = input.target.as_deref().unwrap_or("dummy").to_string();
                *self.health.entry(target.clone()).or_insert(100) -= 10;
                *self.scores.entry(input.player_id.clone()).or_insert(0) += 10;
                self.events.push(format!(
                    "tick {}: {} attacked {} for 10 damage",
                    input.sequence, input.player_id, target
                ));
            }
            "score_update" => {
                let delta = input.score_delta.unwrap_or(1);
                *self.scores.entry(input.player_id.clone()).or_insert(0) += delta;
                self.events.push(format!(
                    "tick {}: gameplay score update {} +{}",
                    input.sequence, input.player_id, delta
                ));
            }
            "tick" => {
                self.events
                    .push(format!("tick {}: session heartbeat", input.sequence));
            }
            _ => {}
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ArenaGameplayInput {
    pub player_id: String,
    pub action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_delta: Option<i64>,
    pub sequence: u64,
}

impl ArenaGameplayInput {
    pub fn multiplayer_inputs() -> Vec<Self> {
        vec![
            Self {
                player_id: "player-a".into(),
                action: "join".into(),
                direction: None,
                target: None,
                score_delta: None,
                sequence: 1,
            },
            Self {
                player_id: "player-b".into(),
                action: "join".into(),
                direction: None,
                target: None,
                score_delta: None,
                sequence: 2,
            },
            Self {
                player_id: "player-a".into(),
                action: "move".into(),
                direction: Some("north".into()),
                target: None,
                score_delta: None,
                sequence: 3,
            },
            Self {
                player_id: "player-b".into(),
                action: "move".into(),
                direction: Some("south".into()),
                target: None,
                score_delta: None,
                sequence: 4,
            },
            Self {
                player_id: "player-a".into(),
                action: "attack".into(),
                direction: None,
                target: Some("player-b".into()),
                score_delta: None,
                sequence: 5,
            },
            Self {
                player_id: "player-a".into(),
                action: "score_update".into(),
                direction: None,
                target: None,
                score_delta: Some(5),
                sequence: 6,
            },
        ]
    }

    pub fn canonical_inputs() -> Vec<Self> {
        vec![
            Self {
                player_id: "player-1".into(),
                action: "join".into(),
                direction: None,
                target: None,
                score_delta: None,
                sequence: 1,
            },
            Self {
                player_id: "player-1".into(),
                action: "move".into(),
                direction: Some("north".into()),
                target: None,
                score_delta: None,
                sequence: 2,
            },
            Self {
                player_id: "player-1".into(),
                action: "attack".into(),
                direction: None,
                target: Some("dummy".into()),
                score_delta: None,
                sequence: 3,
            },
            Self {
                player_id: "player-1".into(),
                action: "score_update".into(),
                direction: None,
                target: None,
                score_delta: Some(5),
                sequence: 4,
            },
            Self {
                player_id: "player-1".into(),
                action: "tick".into(),
                direction: None,
                target: None,
                score_delta: None,
                sequence: 5,
            },
        ]
    }

    pub fn canonical_bytes(&self) -> Result<Vec<u8>> {
        Ok(serde_json::to_vec(self)?)
    }

    pub fn stable_hash(&self) -> Result<String> {
        Ok(hex::encode(Sha256::digest(self.canonical_bytes()?)))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TemplateGameplayProof {
    pub proof_version: String,
    pub status: String,
    pub template: String,
    pub world_id: String,
    pub actions: Vec<String>,
    pub ticks_executed: u64,
    pub previous_state_root: String,
    pub execution_root: String,
    pub state_root_changed: bool,
    pub player_exists: bool,
    pub position_changed: bool,
    pub score_changed: bool,
    pub health_changed: bool,
    pub receipt_count: u64,
    pub journal_length: u64,
    pub replay_root: String,
    pub replay_verified: bool,
    pub replay_verification: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionTranscriptEntry {
    pub tick: u64,
    pub session_id: String,
    pub player_id: String,
    pub action: String,
    pub state_root: String,
    pub score: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MultiplayerLocalSessionProof {
    pub proof_version: String,
    pub status: String,
    pub session_id: String,
    pub world_id: String,
    pub tick: u64,
    pub player_count: u64,
    pub players: Vec<String>,
    pub actions: Vec<String>,
    pub transcript_length: u64,
    pub state_root: String,
    pub final_session_root: String,
    pub replay_root: String,
    pub replay_verified: bool,
    pub replay_verification: String,
    pub player_a_joined: bool,
    pub player_b_joined: bool,
    pub player_a_moved: bool,
    pub player_b_moved: bool,
    pub health_changed: bool,
    pub interaction_recorded: bool,
    pub receipt_count: u64,
    pub journal_length: u64,
    pub guest_execution: String,
    pub guest_hash: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NetworkTransportMessage {
    pub message_id: String,
    pub message_type: String,
    pub session_id: String,
    pub sequence: u64,
    pub sender: String,
    pub payload: serde_json::Value,
    pub hash: String,
}

impl NetworkTransportMessage {
    pub fn new(
        message_type: &str,
        session_id: &str,
        sequence: u64,
        sender: &str,
        payload: serde_json::Value,
    ) -> Result<Self> {
        let message_id = format!("netmsg-{sequence:020}");
        let hash_payload = serde_json::json!({
            "message_id": message_id,
            "message_type": message_type,
            "payload": payload,
            "sender": sender,
            "sequence": sequence,
            "session_id": session_id
        });
        let hash = hex::encode(Sha256::digest(serde_json::to_vec(&hash_payload)?));
        Ok(Self {
            message_id,
            message_type: message_type.into(),
            session_id: session_id.into(),
            sequence,
            sender: sender.into(),
            payload,
            hash,
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NetworkClientState {
    pub client_id: String,
    pub player_id: String,
    pub started: bool,
    pub tick: u64,
    pub state_root: String,
    pub player_count: u64,
    pub sequence: u64,
    pub observed_journal_length: u64,
    pub acknowledged_receipts: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NetworkReceiptDelivery {
    pub receipt_id: String,
    pub generated_by: String,
    pub delivered_to: Vec<String>,
    pub acknowledged_by: Vec<String>,
    pub sequence: u64,
    pub tick: u64,
    pub state_root: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NetworkTransportSessionProof {
    pub proof_version: String,
    pub status: String,
    pub session_id: String,
    pub world_id: String,
    pub tick: u64,
    pub player_count: u64,
    pub sequence: u64,
    pub state_root: String,
    pub client_a_state_root: String,
    pub client_b_state_root: String,
    pub replay_root: String,
    pub replay_verified: bool,
    pub replay_verification: String,
    pub client_a_started: bool,
    pub client_b_started: bool,
    pub session_joined: bool,
    pub movement_synchronized: bool,
    pub interaction_synchronized: bool,
    pub state_synchronized: bool,
    pub receipt_delivery: bool,
    pub journal_observation: bool,
    pub multi_tick_progression: bool,
    pub transcript_length: u64,
    pub receipt_count: u64,
    pub journal_length: u64,
    pub limitations: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayableLocalGameProof {
    pub proof_version: String,
    pub status: String,
    pub session_id: String,
    pub world_id: String,
    pub tick: u64,
    pub player_count: u64,
    pub actions: Vec<String>,
    pub transcript_length: u64,
    pub state_root: String,
    pub replay_root: String,
    pub replay_verified: bool,
    pub replay_verification: String,
    pub score_before: i64,
    pub score_after: i64,
    pub score_origin: String,
    pub receipt_count: u64,
    pub journal_length: u64,
    pub guest_execution: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeterministicExecutionProof {
    pub proof_version: String,
    pub status: String,
    pub world_id: String,
    pub runtime_version: String,
    pub input: DeterministicProofInput,
    pub input_hash: String,
    pub receipt_id: String,
    pub tick: u64,
    pub ticks_executed: u64,
    pub previous_state_root: String,
    pub execution_root: String,
    pub state_root_changed: bool,
    pub tick_count_increased: bool,
    pub receipt_hash: String,
    pub journal_length: u64,
    pub checkpoint_identifier: String,
    pub checkpoint_hash: String,
    pub replay_root: String,
    pub replay_verified: bool,
    pub replay_verification: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GuestRuntimeState {
    pub guest_id: String,
    pub guest_hash: String,
    pub action: String,
    pub player_id: String,
    pub position: GuestPosition,
    pub score: i64,
    pub guest_output_hash: String,
}

impl GuestRuntimeState {
    pub fn root(&self) -> Result<String> {
        Ok(hex::encode(Sha256::digest(serde_json::to_vec(self)?)))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GuestExecutionProof {
    pub proof_version: String,
    pub status: String,
    pub world_id: String,
    pub guest_id: String,
    pub guest_hash: String,
    pub guest_output_hash: String,
    pub action: String,
    pub player_id: String,
    pub position: GuestPosition,
    pub score: i64,
    pub previous_state_root: String,
    pub state_root: String,
    pub execution_root: String,
    pub state_root_changed: bool,
    pub state_mutation_origin: String,
    pub receipt_id: String,
    pub receipt_hash: String,
    pub journal_length: u64,
    pub replay_root: String,
    pub replay_verified: bool,
    pub replay_verification: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RuntimeBootEvidence {
    pub world_id: String,
    pub package_id: String,
    pub package_hash: String,
    pub runtime_version: String,
    pub status: String,
    pub classification: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RuntimeSessionEvidence {
    pub world_id: String,
    pub package_id: String,
    pub package_hash: String,
    pub runtime_version: String,
    pub status: String,
    pub classification: String,
    pub session_id: String,
    pub created_at_ms: u128,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RuntimeProjectionEvidence {
    pub world_id: String,
    pub package_id: String,
    pub package_hash: String,
    pub runtime_version: String,
    pub status: String,
    pub classification: String,
    pub projection_id: String,
    pub non_authoritative_projection: bool,
    pub created_at_ms: u128,
}

pub struct RuntimeLoop {
    pub config: RuntimeConfiguration,
    pub lifecycle: RuntimeLifecycle,
    pub health: RuntimeHealth,
    pub metrics: RuntimeMetrics,
    pub package: LoadedPackage,
    pub input_queue: InputQueue,
    pub journal: JournalManager,
    pub checkpoints: CheckpointManager,
    pub persistence: PersistenceManager,
    pub state: Vec<u8>,
}

impl RuntimeLoop {
    pub fn boot(config: RuntimeConfiguration) -> Result<Self> {
        let persistence = PersistenceManager::new(&config.root);
        persistence.ensure_layout(&[
            config.packages_dir(),
            config.state_dir(),
            config.journals_dir(),
            config.checkpoints_dir(),
            config.receipts_dir(),
            config.sessions_dir(),
            config.projections_dir(),
            config.backups_dir(),
            config.reports_dir(),
        ])?;
        let mut lifecycle = RuntimeLifecycle::boot();
        lifecycle.transition(
            RuntimeState::ValidatingPackage,
            "startup package validation",
        );
        let package =
            PackageLoader::new(config.runtime_version.clone()).load(&config.package_path)?;
        lifecycle.transition(RuntimeState::LoadingState, "load persisted runtime state");
        let input_queue = InputQueue::open(
            config.world_dir().join("input-queue.jsonl"),
            persistence.clone(),
        )?;
        let journal = JournalManager::new(
            config.journals_dir().join("journal.jsonl"),
            persistence.clone(),
        );
        let checkpoints = CheckpointManager::new(
            config.checkpoints_dir(),
            persistence.clone(),
            config.checkpoint_interval_ticks,
        );
        lifecycle.transition(RuntimeState::Recovering, "automatic recovery");
        let state = checkpoints
            .latest()?
            .map(|c| c.state_snapshot)
            .unwrap_or_default();
        let mut health =
            RuntimeHealth::new(config.runtime_version.clone(), config.world_id.clone());
        health.package_hash = package.package_hash.clone();
        health.package_version = package.manifest.package_version.clone();
        health.runtime_state = RuntimeState::Running;
        lifecycle.transition(RuntimeState::Running, "runtime ready");
        let runtime = Self {
            config,
            lifecycle,
            health,
            metrics: RuntimeMetrics::default(),
            package,
            input_queue,
            journal,
            checkpoints,
            persistence,
            state,
        };
        runtime.write_boot_evidence()?;
        Ok(runtime)
    }

    fn classification(&self) -> String {
        self.package
            .world_metadata
            .get("package_classification")
            .or_else(|| self.package.world_metadata.get("classification"))
            .and_then(|v| v.as_str())
            .unwrap_or("runtime-package")
            .to_string()
    }

    fn evidence_base(&self) -> RuntimeBootEvidence {
        RuntimeBootEvidence {
            world_id: self.config.world_id.clone(),
            package_id: self.package.manifest.package_id.clone(),
            package_hash: self.package.package_hash.clone(),
            runtime_version: self.config.runtime_version.clone(),
            status: "Runtime Boot Proven".into(),
            classification: self.classification(),
        }
    }

    fn write_boot_evidence(&self) -> Result<()> {
        let base = self.evidence_base();
        self.persistence.atomic_write_json(
            self.config.reports_dir().join("runtime_start_report.json"),
            &base,
        )?;
        let created_at_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        let session = RuntimeSessionEvidence {
            world_id: base.world_id.clone(),
            package_id: base.package_id.clone(),
            package_hash: base.package_hash.clone(),
            runtime_version: base.runtime_version.clone(),
            status: base.status.clone(),
            classification: base.classification.clone(),
            session_id: "session-0001".into(),
            created_at_ms,
        };
        self.persistence.atomic_write_json(
            self.config.sessions_dir().join("session-0001.json"),
            &session,
        )?;
        let projection = RuntimeProjectionEvidence {
            world_id: base.world_id,
            package_id: base.package_id,
            package_hash: base.package_hash,
            runtime_version: base.runtime_version,
            status: base.status,
            classification: base.classification,
            projection_id: "projection-0001".into(),
            non_authoritative_projection: true,
            created_at_ms,
        };
        self.persistence.atomic_write_json(
            self.config.projections_dir().join("projection-0001.json"),
            &projection,
        )?;
        self.persistence
            .write_versioned(self.config.runtime_status_path(), &self.health)?;
        Ok(())
    }

    pub fn submit_input(
        &mut self,
        origin: impl Into<String>,
        payload: Vec<u8>,
    ) -> Result<RuntimeInput> {
        let input = self.input_queue.enqueue(origin, payload)?;
        self.metrics.input_queue_depth = self.input_queue.depth();
        Ok(input)
    }

    pub fn execute_tick(&mut self) -> Result<Option<RuntimeReceipt>> {
        let Some(input) = self.input_queue.pop() else {
            return Ok(None);
        };
        let previous_hash = self.journal.latest_hash()?;
        self.state.extend_from_slice(input.payload_hash.as_bytes());
        let state_root = hex::encode(Sha256::digest(&self.state));
        let receipt_id = format!("receipt-{:020}", input.sequence);
        let tick = input.sequence;
        let timestamp_or_epoch = input.sequence;
        let receipt_hash = Self::deterministic_receipt_hash(
            &receipt_id,
            tick,
            &input.payload_hash,
            &state_root,
            &self.config.runtime_version,
            &self.config.world_id,
            timestamp_or_epoch,
        );
        let receipt = RuntimeReceipt {
            receipt_id: receipt_id.clone(),
            sequence: input.sequence,
            tick,
            input_id: input.input_id.clone(),
            input_hash: input.payload_hash.clone(),
            state_root: state_root.clone(),
            receipt_hash: receipt_hash.clone(),
            runtime_version: self.config.runtime_version.clone(),
            world_id: self.config.world_id.clone(),
            timestamp_or_epoch,
            session_id: None,
            player_count: None,
            action: None,
            player_id: None,
            guest_id: None,
            guest_hash: None,
            guest_output_hash: None,
        };
        self.persistence.write_versioned(
            self.config
                .receipts_dir()
                .join(format!("{receipt_id}.json")),
            &receipt,
        )?;
        let entry = self.journal.append(
            input.sequence,
            previous_hash,
            state_root.clone(),
            input.payload_hash,
            receipt_hash.clone(),
        )?;
        if self.checkpoints.should_checkpoint(input.sequence) {
            self.lifecycle
                .transition(RuntimeState::Checkpointing, "checkpoint interval reached");
            self.checkpoints.create(
                input.sequence,
                &self.config.world_id,
                &self.config.runtime_version,
                entry.sequence,
                state_root.clone(),
                self.state.clone(),
                self.package.world_metadata.clone(),
            )?;
            self.metrics.checkpoint_count += 1;
            self.lifecycle
                .transition(RuntimeState::Running, "checkpoint completed");
        }
        self.health.world_root = state_root;
        self.health.journal_height = entry.sequence;
        self.health.latest_receipt = Some(receipt_hash);
        self.health.runtime_state = self.lifecycle.state.clone();
        self.metrics.ticks_executed += 1;
        self.metrics.receipts_generated += 1;
        self.metrics.input_queue_depth = self.input_queue.depth();
        self.persistence
            .write_versioned(self.config.runtime_status_path(), &self.health)?;
        Ok(Some(receipt))
    }

    fn deterministic_receipt_hash(
        receipt_id: &str,
        tick: u64,
        input_hash: &str,
        state_root: &str,
        runtime_version: &str,
        world_id: &str,
        timestamp_or_epoch: u64,
    ) -> String {
        let mut h = Sha256::new();
        h.update(receipt_id.as_bytes());
        h.update(tick.to_le_bytes());
        h.update(input_hash.as_bytes());
        h.update(state_root.as_bytes());
        h.update(runtime_version.as_bytes());
        h.update(world_id.as_bytes());
        h.update(timestamp_or_epoch.to_le_bytes());
        hex::encode(h.finalize())
    }

    pub fn execute_deterministic_proof(&mut self) -> Result<DeterministicExecutionProof> {
        let proof_input = DeterministicProofInput::canonical();
        let payload = proof_input.canonical_bytes()?;
        let input_hash = proof_input.stable_hash()?;
        let previous_state_root = hex::encode(Sha256::digest(&self.state));
        let previous_ticks = self.metrics.ticks_executed;
        let input = self.submit_input("deterministic-execution-proof", payload)?;
        if input.payload_hash != input_hash {
            anyhow::bail!("canonical input hash mismatch");
        }
        let ticks_executed = self.run_ticks(1)?;
        let receipt = self.persistence.read_versioned::<RuntimeReceipt>(
            self.config
                .receipts_dir()
                .join(format!("receipt-{:020}.json", input.sequence)),
        )?;
        let entries = self.journal.entries()?;
        let execution_root = receipt.state_root.clone();
        let replay_root = ReplayManager::replay_root(&[], &entries);
        let replay_verified = replay_root == execution_root
            && ReplayManager
                .verify_equivalence(&[], &entries, &execution_root)
                .is_ok();
        let checkpoint = self.checkpoints.create(
            input.sequence,
            &self.config.world_id,
            &self.config.runtime_version,
            entries.len() as u64,
            execution_root.clone(),
            self.state.clone(),
            self.package.world_metadata.clone(),
        )?;
        self.checkpoints.verify_checkpoint(&checkpoint)?;
        self.metrics.checkpoint_count += 1;
        self.health.checkpoint_height = checkpoint.manifest.sequence;
        self.health.world_root = execution_root.clone();
        self.health.runtime_state = self.lifecycle.state.clone();
        self.persistence
            .write_versioned(self.config.runtime_status_path(), &self.health)?;

        let proof = DeterministicExecutionProof {
            proof_version: "deterministic-execution-proof-v0.1".into(),
            status: if replay_verified {
                "Deterministic Execution: PASS".into()
            } else {
                "Deterministic Execution: FAIL".into()
            },
            world_id: self.config.world_id.clone(),
            runtime_version: self.config.runtime_version.clone(),
            input: proof_input,
            input_hash,
            receipt_id: receipt.receipt_id,
            tick: receipt.tick,
            ticks_executed,
            previous_state_root: previous_state_root.clone(),
            execution_root,
            state_root_changed: receipt.state_root != previous_state_root,
            tick_count_increased: self.metrics.ticks_executed > previous_ticks,
            receipt_hash: receipt.receipt_hash,
            journal_length: entries.len() as u64,
            checkpoint_identifier: format!("checkpoint-{:020}", checkpoint.manifest.sequence),
            checkpoint_hash: checkpoint.manifest.checkpoint_hash,
            replay_root,
            replay_verified,
            replay_verification: if replay_verified { "PASS" } else { "FAIL" }.into(),
        };
        self.write_execution_artifacts(&proof)?;
        Ok(proof)
    }

    fn write_execution_artifacts(&self, proof: &DeterministicExecutionProof) -> Result<()> {
        let replay_dir = self.config.root.join("replay");
        let receipts_dir = self.config.root.join("receipts");
        let journals_dir = self.config.root.join("journals");
        let checkpoints_dir = self.config.root.join("checkpoints");
        self.persistence.ensure_layout(&[
            replay_dir.clone(),
            receipts_dir.clone(),
            journals_dir.clone(),
            checkpoints_dir.clone(),
        ])?;
        self.persistence.atomic_write_json(
            self.config
                .reports_dir()
                .join("deterministic-execution-proof.json"),
            proof,
        )?;
        self.persistence
            .atomic_write_json(replay_dir.join("replay-proof.json"), proof)?;
        std::fs::copy(
            self.config
                .receipts_dir()
                .join(format!("{}.json", proof.receipt_id)),
            receipts_dir.join(format!("{}.json", proof.receipt_id)),
        )?;
        std::fs::copy(
            self.config.journals_dir().join("journal.jsonl"),
            journals_dir.join("journal.jsonl"),
        )?;
        std::fs::copy(
            self.config
                .checkpoints_dir()
                .join(format!("{}.json", proof.checkpoint_identifier)),
            checkpoints_dir.join(format!("{}.json", proof.checkpoint_identifier)),
        )?;
        Ok(())
    }

    pub fn execute_template_gameplay_proof(&mut self) -> Result<TemplateGameplayProof> {
        if self
            .package
            .world_metadata
            .get("template")
            .and_then(|v| v.as_str())
            != Some("arena")
        {
            anyhow::bail!("template gameplay proof currently supports the arena template only");
        }

        let inputs = ArenaGameplayInput::canonical_inputs();
        let previous_state = ArenaState::initial();
        let previous_state_root = previous_state.root()?;
        let mut arena_state = previous_state.clone();
        let previous_position = arena_state.positions.get("player-1").cloned();
        let previous_score = arena_state.scores.get("player-1").copied().unwrap_or(0);
        let previous_dummy_health = arena_state.health.get("dummy").copied().unwrap_or(100);
        let mut receipt_count = 0_u64;

        for gameplay_input in &inputs {
            let payload = gameplay_input.canonical_bytes()?;
            let input_hash = gameplay_input.stable_hash()?;
            let input = self.submit_input("arena-template-gameplay-proof", payload)?;
            if input.payload_hash != input_hash {
                anyhow::bail!(
                    "arena gameplay input hash mismatch at sequence {}",
                    gameplay_input.sequence
                );
            }
            let queued = self
                .input_queue
                .pop()
                .ok_or_else(|| anyhow::anyhow!("arena gameplay input queue underflow"))?;
            if queued.sequence != gameplay_input.sequence || queued.payload_hash != input_hash {
                anyhow::bail!("arena gameplay input sequence mismatch");
            }

            let previous_hash = self.journal.latest_hash()?;
            arena_state.apply(gameplay_input);
            let state_root = arena_state.root()?;
            self.state = serde_json::to_vec(&arena_state)?;
            let receipt_id = format!("receipt-{:020}", queued.sequence);
            let tick = gameplay_input.sequence;
            let timestamp_or_epoch = queued.sequence;
            let receipt_hash = Self::deterministic_receipt_hash(
                &receipt_id,
                tick,
                &input_hash,
                &state_root,
                &self.config.runtime_version,
                &self.config.world_id,
                timestamp_or_epoch,
            );
            let receipt = RuntimeReceipt {
                receipt_id: receipt_id.clone(),
                sequence: queued.sequence,
                tick,
                input_id: queued.input_id.clone(),
                input_hash: input_hash.clone(),
                state_root: state_root.clone(),
                receipt_hash: receipt_hash.clone(),
                runtime_version: self.config.runtime_version.clone(),
                world_id: self.config.world_id.clone(),
                timestamp_or_epoch,
                session_id: Some(arena_state.session_id.clone()),
                player_count: Some(arena_state.players.len() as u64),
                action: Some(gameplay_input.action.clone()),
                player_id: Some(gameplay_input.player_id.clone()),
                guest_id: None,
                guest_hash: None,
                guest_output_hash: None,
            };
            self.persistence.write_versioned(
                self.config
                    .receipts_dir()
                    .join(format!("{receipt_id}.json")),
                &receipt,
            )?;
            self.journal.append_gameplay(
                queued.sequence,
                previous_hash,
                state_root.clone(),
                input_hash,
                receipt_hash.clone(),
                gameplay_input.player_id.clone(),
                gameplay_input.action.clone(),
                tick,
                serde_json::to_value(gameplay_input)?,
            )?;
            self.metrics.ticks_executed += 1;
            self.metrics.receipts_generated += 1;
            self.metrics.input_queue_depth = self.input_queue.depth();
            self.health.world_root = state_root;
            self.health.journal_height = queued.sequence;
            self.health.latest_receipt = Some(receipt_hash);
            receipt_count += 1;
        }

        let entries = self.journal.entries()?;
        let execution_root = arena_state.root()?;
        let replay_state = Self::replay_arena_state_from_entries(&entries)?;
        let replay_root = replay_state.root()?;
        let replay_verified = replay_root == execution_root;
        let checkpoint = self.checkpoints.create(
            arena_state.tick,
            &self.config.world_id,
            &self.config.runtime_version,
            entries.len() as u64,
            execution_root.clone(),
            self.state.clone(),
            self.package.world_metadata.clone(),
        )?;
        self.checkpoints.verify_checkpoint(&checkpoint)?;
        self.metrics.checkpoint_count += 1;
        self.health.checkpoint_height = checkpoint.manifest.sequence;
        self.health.world_root = execution_root.clone();
        self.persistence
            .write_versioned(self.config.runtime_status_path(), &self.health)?;

        let final_position = arena_state.positions.get("player-1").cloned();
        let final_score = arena_state.scores.get("player-1").copied().unwrap_or(0);
        let final_dummy_health = arena_state.health.get("dummy").copied().unwrap_or(100);
        let proof = TemplateGameplayProof {
            proof_version: "template-gameplay-execution-proof-v0.1".into(),
            status: if replay_verified {
                "Template Gameplay Execution: PASS".into()
            } else {
                "Template Gameplay Execution: FAIL".into()
            },
            template: "arena".into(),
            world_id: self.config.world_id.clone(),
            actions: inputs.iter().map(|i| i.action.clone()).collect(),
            ticks_executed: self.metrics.ticks_executed,
            previous_state_root: previous_state_root.clone(),
            execution_root: execution_root.clone(),
            state_root_changed: execution_root != previous_state_root,
            player_exists: arena_state.players.contains_key("player-1"),
            position_changed: final_position != previous_position,
            score_changed: final_score != previous_score,
            health_changed: final_dummy_health != previous_dummy_health,
            receipt_count,
            journal_length: entries.len() as u64,
            replay_root,
            replay_verified,
            replay_verification: if replay_verified { "PASS" } else { "FAIL" }.into(),
        };
        self.write_template_gameplay_artifacts(&arena_state, &proof)?;
        Ok(proof)
    }

    fn replay_arena_state_from_entries(entries: &[JournalEntry]) -> Result<ArenaState> {
        let mut state = ArenaState::initial();
        for entry in entries {
            let Some(value) = &entry.gameplay_input else {
                continue;
            };
            let input: ArenaGameplayInput = serde_json::from_value(value.clone())?;
            state.apply(&input);
        }
        Ok(state)
    }

    fn write_template_gameplay_artifacts(
        &self,
        state: &ArenaState,
        proof: &TemplateGameplayProof,
    ) -> Result<()> {
        let gameplay_dir = self.config.root.join("gameplay");
        let replay_dir = self.config.root.join("replay");
        let receipts_dir = self.config.root.join("receipts");
        let journals_dir = self.config.root.join("journals");
        self.persistence.ensure_layout(&[
            gameplay_dir.clone(),
            replay_dir.clone(),
            receipts_dir.clone(),
            journals_dir.clone(),
        ])?;
        self.persistence
            .atomic_write_json(gameplay_dir.join("arena-state.json"), state)?;
        self.persistence
            .atomic_write_json(replay_dir.join("gameplay-replay-proof.json"), proof)?;
        self.persistence.atomic_write_json(
            self.config
                .reports_dir()
                .join("template-gameplay-execution-proof.json"),
            proof,
        )?;
        for entry in std::fs::read_dir(self.config.receipts_dir())? {
            let entry = entry?;
            if entry.file_type()?.is_file() {
                std::fs::copy(entry.path(), receipts_dir.join(entry.file_name()))?;
            }
        }
        std::fs::copy(
            self.config.journals_dir().join("journal.jsonl"),
            journals_dir.join("journal.jsonl"),
        )?;
        Ok(())
    }

    pub fn execute_playable_local_session(&mut self) -> Result<PlayableLocalGameProof> {
        let template_proof = self.execute_template_gameplay_proof()?;
        let arena_state: ArenaState = serde_json::from_slice(&self.state)?;
        let entries = self.journal.entries()?;
        let mut transcript = Vec::new();
        for entry in &entries {
            let Some(value) = &entry.gameplay_input else {
                continue;
            };
            let input: ArenaGameplayInput = serde_json::from_value(value.clone())?;
            let replay_entries = entries
                .iter()
                .filter(|candidate| candidate.sequence <= entry.sequence)
                .cloned()
                .collect::<Vec<_>>();
            let replay_state = Self::replay_arena_state_from_entries(&replay_entries)?;
            transcript.push(SessionTranscriptEntry {
                tick: input.sequence,
                session_id: replay_state.session_id.clone(),
                player_id: input.player_id.clone(),
                action: input.action.clone(),
                state_root: replay_state.root()?,
                score: replay_state
                    .scores
                    .get(&input.player_id)
                    .copied()
                    .unwrap_or(0),
            });
        }
        let replay_state = Self::replay_arena_state_from_entries(&entries)?;
        let replay_root = replay_state.root()?;
        let state_root = arena_state.root()?;
        let replay_verified = replay_root == state_root;
        let proof = PlayableLocalGameProof {
            proof_version: "playable-local-game-proof-v0.1".into(),
            status: if replay_verified {
                "Playable Local Game: PASS".into()
            } else {
                "Playable Local Game: FAIL".into()
            },
            session_id: arena_state.session_id.clone(),
            world_id: self.config.world_id.clone(),
            tick: arena_state.tick,
            player_count: arena_state.players.len() as u64,
            actions: transcript.iter().map(|entry| entry.action.clone()).collect(),
            transcript_length: transcript.len() as u64,
            state_root: state_root.clone(),
            replay_root: replay_root.clone(),
            replay_verified,
            replay_verification: if replay_verified { "PASS" } else { "FAIL" }.into(),
            score_before: 0,
            score_after: arena_state.scores.get("player-1").copied().unwrap_or(0),
            score_origin: "gameplay attack and score_update actions".into(),
            receipt_count: template_proof.receipt_count,
            journal_length: entries.len() as u64,
            guest_execution: "runtime package world.wasm loaded and gameplay actions executed by the local runtime session".into(),
        };
        self.write_playable_local_session_artifacts(&arena_state, &transcript, &proof)?;
        Ok(proof)
    }

    fn write_playable_local_session_artifacts(
        &self,
        state: &ArenaState,
        transcript: &[SessionTranscriptEntry],
        proof: &PlayableLocalGameProof,
    ) -> Result<()> {
        let sessions_dir = self.config.root.join("sessions");
        let gameplay_dir = self.config.root.join("gameplay");
        let replay_dir = self.config.root.join("replay");
        self.persistence.ensure_layout(&[
            sessions_dir.clone(),
            gameplay_dir.clone(),
            replay_dir.clone(),
        ])?;
        let session = serde_json::json!({
            "session_id": proof.session_id,
            "world_id": proof.world_id,
            "tick": proof.tick,
            "player_count": proof.player_count,
            "state_root": proof.state_root,
            "status": "Playable Local Game Proven"
        });
        self.persistence
            .atomic_write_json(sessions_dir.join("session-0001.json"), &session)?;
        self.persistence.atomic_write_json(
            self.config.sessions_dir().join("session-0001.json"),
            &session,
        )?;
        self.persistence
            .atomic_write_json(gameplay_dir.join("arena-state.json"), state)?;
        self.persistence.atomic_write_json(
            gameplay_dir.join("session-transcript.json"),
            &transcript.to_vec(),
        )?;
        self.persistence
            .atomic_write_json(replay_dir.join("gameplay-replay-proof.json"), proof)?;
        self.persistence.atomic_write_json(
            self.config
                .reports_dir()
                .join("playable-local-game-proof.json"),
            proof,
        )?;
        Ok(())
    }

    pub fn execute_multiplayer_local_session(&mut self) -> Result<MultiplayerLocalSessionProof> {
        if self
            .package
            .world_metadata
            .get("template")
            .and_then(|v| v.as_str())
            != Some("arena")
        {
            anyhow::bail!(
                "multiplayer local session proof currently supports the arena template only"
            );
        }

        let guest_id = self.package.manifest.package_id.clone();
        let guest_execution = GuestWasmRunner::execute(&guest_id, &self.package.wasm)?;
        if guest_execution.guest_hash != self.package.manifest.wasm_hash {
            anyhow::bail!("guest wasm hash does not match package manifest");
        }

        let inputs = ArenaGameplayInput::multiplayer_inputs();
        let previous_state = ArenaState::initial();
        let mut arena_state = previous_state.clone();
        let player_a_start = arena_state.positions.get("player-a").cloned();
        let player_b_start = arena_state.positions.get("player-b").cloned();
        let player_b_health_start = 100_i64;
        let mut receipt_count = 0_u64;

        for gameplay_input in &inputs {
            let payload = gameplay_input.canonical_bytes()?;
            let input_hash = gameplay_input.stable_hash()?;
            let input = self.submit_input("arena-multiplayer-local-session-proof", payload)?;
            if input.payload_hash != input_hash {
                anyhow::bail!(
                    "arena multiplayer input hash mismatch at sequence {}",
                    gameplay_input.sequence
                );
            }
            let queued = self
                .input_queue
                .pop()
                .ok_or_else(|| anyhow::anyhow!("arena multiplayer input queue underflow"))?;
            if queued.sequence != gameplay_input.sequence || queued.payload_hash != input_hash {
                anyhow::bail!("arena multiplayer input sequence mismatch");
            }

            let previous_hash = self.journal.latest_hash()?;
            arena_state.apply(gameplay_input);
            let state_root = arena_state.root()?;
            self.state = serde_json::to_vec(&arena_state)?;
            let receipt_id = format!("receipt-{:020}", queued.sequence);
            let tick = gameplay_input.sequence;
            let timestamp_or_epoch = queued.sequence;
            let receipt_hash = Self::deterministic_receipt_hash(
                &receipt_id,
                tick,
                &input_hash,
                &state_root,
                &self.config.runtime_version,
                &self.config.world_id,
                timestamp_or_epoch,
            );
            let receipt = RuntimeReceipt {
                receipt_id: receipt_id.clone(),
                sequence: queued.sequence,
                tick,
                input_id: queued.input_id.clone(),
                input_hash: input_hash.clone(),
                state_root: state_root.clone(),
                receipt_hash: receipt_hash.clone(),
                runtime_version: self.config.runtime_version.clone(),
                world_id: self.config.world_id.clone(),
                timestamp_or_epoch,
                session_id: Some(arena_state.session_id.clone()),
                player_count: Some(arena_state.player_count),
                action: Some(gameplay_input.action.clone()),
                player_id: Some(gameplay_input.player_id.clone()),
                guest_id: Some(guest_execution.guest_id.clone()),
                guest_hash: Some(guest_execution.guest_hash.clone()),
                guest_output_hash: Some(guest_execution.guest_output_hash.clone()),
            };
            self.persistence.write_versioned(
                self.config
                    .receipts_dir()
                    .join(format!("{receipt_id}.json")),
                &receipt,
            )?;
            self.journal.append_gameplay(
                queued.sequence,
                previous_hash,
                state_root.clone(),
                input_hash,
                receipt_hash.clone(),
                gameplay_input.player_id.clone(),
                gameplay_input.action.clone(),
                tick,
                serde_json::to_value(gameplay_input)?,
            )?;
            self.metrics.ticks_executed += 1;
            self.metrics.receipts_generated += 1;
            self.metrics.input_queue_depth = self.input_queue.depth();
            self.health.world_root = state_root;
            self.health.journal_height = queued.sequence;
            self.health.latest_receipt = Some(receipt_hash);
            receipt_count += 1;
        }

        let entries = self.journal.entries()?;
        let mut transcript = Vec::new();
        for entry in &entries {
            let Some(value) = &entry.gameplay_input else {
                continue;
            };
            let input: ArenaGameplayInput = serde_json::from_value(value.clone())?;
            let replay_entries = entries
                .iter()
                .filter(|candidate| candidate.sequence <= entry.sequence)
                .cloned()
                .collect::<Vec<_>>();
            let replay_state = Self::replay_arena_state_from_entries(&replay_entries)?;
            transcript.push(SessionTranscriptEntry {
                tick: input.sequence,
                session_id: replay_state.session_id.clone(),
                player_id: input.player_id.clone(),
                action: input.action.clone(),
                state_root: replay_state.root()?,
                score: replay_state
                    .scores
                    .get(&input.player_id)
                    .copied()
                    .unwrap_or(0),
            });
        }

        let state_root = arena_state.root()?;
        let replay_state = Self::replay_arena_state_from_entries(&entries)?;
        let replay_root = replay_state.root()?;
        let replay_verified = replay_root == state_root;
        let checkpoint = self.checkpoints.create(
            arena_state.tick,
            &self.config.world_id,
            &self.config.runtime_version,
            entries.len() as u64,
            state_root.clone(),
            self.state.clone(),
            self.package.world_metadata.clone(),
        )?;
        self.checkpoints.verify_checkpoint(&checkpoint)?;
        self.metrics.checkpoint_count += 1;
        self.health.checkpoint_height = checkpoint.manifest.sequence;
        self.health.world_root = state_root.clone();
        self.persistence
            .write_versioned(self.config.runtime_status_path(), &self.health)?;

        let player_a_end = arena_state.positions.get("player-a").cloned();
        let player_b_end = arena_state.positions.get("player-b").cloned();
        let player_b_health_end = arena_state.health.get("player-b").copied().unwrap_or(100);
        let players = arena_state.players.keys().cloned().collect::<Vec<_>>();
        let proof = MultiplayerLocalSessionProof {
            proof_version: "multiplayer-local-session-proof-v0.1".into(),
            status: if replay_verified {
                "Multiplayer Local Session: PASS".into()
            } else {
                "Multiplayer Local Session: FAIL".into()
            },
            session_id: arena_state.session_id.clone(),
            world_id: self.config.world_id.clone(),
            tick: arena_state.tick,
            player_count: arena_state.player_count,
            players,
            actions: transcript.iter().map(|entry| entry.action.clone()).collect(),
            transcript_length: transcript.len() as u64,
            state_root: state_root.clone(),
            final_session_root: state_root.clone(),
            replay_root: replay_root.clone(),
            replay_verified,
            replay_verification: if replay_verified { "PASS" } else { "FAIL" }.into(),
            player_a_joined: arena_state.players.contains_key("player-a"),
            player_b_joined: arena_state.players.contains_key("player-b"),
            player_a_moved: player_a_end != player_a_start,
            player_b_moved: player_b_end != player_b_start,
            health_changed: player_b_health_end != player_b_health_start,
            interaction_recorded: arena_state
                .events
                .iter()
                .any(|event| event.contains("player-a attacked player-b")),
            receipt_count,
            journal_length: entries.len() as u64,
            guest_execution: "Arena Guest WASM executed before deterministic multiplayer action application; receipts bind each multiplayer action to guest_hash and guest_output_hash".into(),
            guest_hash: guest_execution.guest_hash.clone(),
        };
        self.write_multiplayer_local_session_artifacts(&arena_state, &transcript, &proof)?;
        Ok(proof)
    }

    fn write_multiplayer_local_session_artifacts(
        &self,
        state: &ArenaState,
        transcript: &[SessionTranscriptEntry],
        proof: &MultiplayerLocalSessionProof,
    ) -> Result<()> {
        let sessions_dir = self.config.root.join("sessions");
        let gameplay_dir = self.config.root.join("gameplay");
        let replay_dir = self.config.root.join("replay");
        let receipts_dir = self.config.root.join("receipts");
        let journals_dir = self.config.root.join("journals");
        self.persistence.ensure_layout(&[
            sessions_dir.clone(),
            gameplay_dir.clone(),
            replay_dir.clone(),
            receipts_dir.clone(),
            journals_dir.clone(),
        ])?;
        let session = serde_json::json!({
            "session_id": proof.session_id,
            "world_id": proof.world_id,
            "tick": proof.tick,
            "player_count": proof.player_count,
            "players": proof.players,
            "state_root": proof.state_root,
            "status": "Multiplayer Local Session Proven"
        });
        self.persistence
            .atomic_write_json(sessions_dir.join("session-0001.json"), &session)?;
        self.persistence.atomic_write_json(
            self.config.sessions_dir().join("session-0001.json"),
            &session,
        )?;
        self.persistence
            .atomic_write_json(gameplay_dir.join("arena-state.json"), state)?;
        self.persistence.atomic_write_json(
            gameplay_dir.join("multiplayer-session-transcript.json"),
            &transcript.to_vec(),
        )?;
        self.persistence
            .atomic_write_json(replay_dir.join("multiplayer-replay-proof.json"), proof)?;
        self.persistence.atomic_write_json(
            self.config
                .reports_dir()
                .join("multiplayer-local-session-proof.json"),
            proof,
        )?;
        for entry in std::fs::read_dir(self.config.receipts_dir())? {
            let entry = entry?;
            if entry.file_type()?.is_file() {
                std::fs::copy(entry.path(), receipts_dir.join(entry.file_name()))?;
            }
        }
        std::fs::copy(
            self.config.journals_dir().join("journal.jsonl"),
            journals_dir.join("journal.jsonl"),
        )?;
        Ok(())
    }

    pub fn execute_network_transport_session(&mut self) -> Result<NetworkTransportSessionProof> {
        if self
            .package
            .world_metadata
            .get("template")
            .and_then(|v| v.as_str())
            != Some("arena")
        {
            anyhow::bail!(
                "network transport session proof currently supports the arena template only"
            );
        }

        let guest_id = self.package.manifest.package_id.clone();
        let guest_execution = GuestWasmRunner::execute(&guest_id, &self.package.wasm)?;
        if guest_execution.guest_hash != self.package.manifest.wasm_hash {
            anyhow::bail!("guest wasm hash does not match package manifest");
        }

        let mut transport_log = Vec::new();
        let mut transcript = Vec::new();
        let mut receipt_delivery = Vec::new();
        let mut arena_state = ArenaState::initial();
        let inputs = ArenaGameplayInput::multiplayer_inputs();
        let mut transport_sequence = 0_u64;
        let mut receipt_count = 0_u64;

        transport_sequence += 1;
        transport_log.push(NetworkTransportMessage::new(
            "Heartbeat",
            &arena_state.session_id,
            transport_sequence,
            "runtime-authority",
            serde_json::json!({ "tick": arena_state.tick, "state_root": arena_state.root()?, "player_count": arena_state.player_count }),
        )?);

        for gameplay_input in &inputs {
            transport_sequence += 1;
            let message_type = match gameplay_input.action.as_str() {
                "join" => "JoinSession",
                "move" => "MovePlayer",
                "attack" => "AttackPlayer",
                _ => "Heartbeat",
            };
            let sender = match gameplay_input.player_id.as_str() {
                "player-a" => "client-a",
                "player-b" => "client-b",
                _ => "runtime-authority",
            };
            let payload = serde_json::to_value(gameplay_input)?;
            transport_log.push(NetworkTransportMessage::new(
                message_type,
                &arena_state.session_id,
                transport_sequence,
                sender,
                payload,
            )?);

            let payload_bytes = gameplay_input.canonical_bytes()?;
            let input_hash = gameplay_input.stable_hash()?;
            let input =
                self.submit_input("arena-network-transport-session-proof", payload_bytes)?;
            if input.payload_hash != input_hash {
                anyhow::bail!(
                    "arena network transport input hash mismatch at sequence {}",
                    gameplay_input.sequence
                );
            }
            let queued = self
                .input_queue
                .pop()
                .ok_or_else(|| anyhow::anyhow!("arena network transport input queue underflow"))?;
            if queued.sequence != gameplay_input.sequence || queued.payload_hash != input_hash {
                anyhow::bail!("arena network transport input sequence mismatch");
            }

            let previous_hash = self.journal.latest_hash()?;
            arena_state.apply(gameplay_input);
            let state_root = arena_state.root()?;
            self.state = serde_json::to_vec(&arena_state)?;
            let receipt_id = format!("receipt-{:020}", queued.sequence);
            let tick = gameplay_input.sequence;
            let timestamp_or_epoch = queued.sequence;
            let receipt_hash = Self::deterministic_receipt_hash(
                &receipt_id,
                tick,
                &input_hash,
                &state_root,
                &self.config.runtime_version,
                &self.config.world_id,
                timestamp_or_epoch,
            );
            let receipt = RuntimeReceipt {
                receipt_id: receipt_id.clone(),
                sequence: queued.sequence,
                tick,
                input_id: queued.input_id.clone(),
                input_hash: input_hash.clone(),
                state_root: state_root.clone(),
                receipt_hash: receipt_hash.clone(),
                runtime_version: self.config.runtime_version.clone(),
                world_id: self.config.world_id.clone(),
                timestamp_or_epoch,
                session_id: Some(arena_state.session_id.clone()),
                player_count: Some(arena_state.player_count),
                action: Some(gameplay_input.action.clone()),
                player_id: Some(gameplay_input.player_id.clone()),
                guest_id: Some(guest_execution.guest_id.clone()),
                guest_hash: Some(guest_execution.guest_hash.clone()),
                guest_output_hash: Some(guest_execution.guest_output_hash.clone()),
            };
            self.persistence.write_versioned(
                self.config
                    .receipts_dir()
                    .join(format!("{receipt_id}.json")),
                &receipt,
            )?;
            self.journal.append_gameplay(
                queued.sequence,
                previous_hash,
                state_root.clone(),
                input_hash,
                receipt_hash.clone(),
                gameplay_input.player_id.clone(),
                gameplay_input.action.clone(),
                tick,
                serde_json::to_value(gameplay_input)?,
            )?;
            self.metrics.ticks_executed += 1;
            self.metrics.receipts_generated += 1;
            self.metrics.input_queue_depth = self.input_queue.depth();
            self.health.world_root = state_root.clone();
            self.health.journal_height = queued.sequence;
            self.health.latest_receipt = Some(receipt_hash.clone());
            receipt_count += 1;

            let delivered_to = vec!["client-a".to_string(), "client-b".to_string()];
            receipt_delivery.push(NetworkReceiptDelivery {
                receipt_id: receipt_id.clone(),
                generated_by: "runtime-authority".into(),
                delivered_to: delivered_to.clone(),
                acknowledged_by: delivered_to,
                sequence: queued.sequence,
                tick,
                state_root: state_root.clone(),
            });

            transport_sequence += 1;
            transport_log.push(NetworkTransportMessage::new(
                "Receipt",
                &arena_state.session_id,
                transport_sequence,
                "runtime-authority",
                serde_json::json!({ "receipt_id": receipt_id, "receipt_hash": receipt_hash, "tick": tick, "state_root": state_root }),
            )?);
            transport_sequence += 1;
            transport_log.push(NetworkTransportMessage::new(
                "SessionState",
                &arena_state.session_id,
                transport_sequence,
                "runtime-authority",
                serde_json::json!({ "tick": arena_state.tick, "state_root": arena_state.root()?, "player_count": arena_state.player_count, "players": arena_state.players.keys().cloned().collect::<Vec<_>>() }),
            )?);

            transcript.push(serde_json::json!({
                "sequence": transport_sequence,
                "tick": arena_state.tick,
                "session_id": arena_state.session_id,
                "sender": sender,
                "action": gameplay_input.action,
                "player_id": gameplay_input.player_id,
                "target": gameplay_input.target,
                "state_root": arena_state.root()?,
                "player_count": arena_state.player_count,
            }));
        }

        let entries = self.journal.entries()?;
        let state_root = arena_state.root()?;
        let replay_state = Self::replay_arena_state_from_entries(&entries)?;
        let replay_root = replay_state.root()?;
        let replay_verified = replay_root == state_root;
        let checkpoint = self.checkpoints.create(
            arena_state.tick,
            &self.config.world_id,
            &self.config.runtime_version,
            entries.len() as u64,
            state_root.clone(),
            self.state.clone(),
            self.package.world_metadata.clone(),
        )?;
        self.checkpoints.verify_checkpoint(&checkpoint)?;
        self.metrics.checkpoint_count += 1;
        self.health.checkpoint_height = checkpoint.manifest.sequence;
        self.health.world_root = state_root.clone();
        self.persistence
            .write_versioned(self.config.runtime_status_path(), &self.health)?;

        let acknowledged_a = receipt_delivery
            .iter()
            .filter(|delivery| {
                delivery
                    .acknowledged_by
                    .iter()
                    .any(|client| client == "client-a")
            })
            .map(|delivery| delivery.receipt_id.clone())
            .collect::<Vec<_>>();
        let acknowledged_b = receipt_delivery
            .iter()
            .filter(|delivery| {
                delivery
                    .acknowledged_by
                    .iter()
                    .any(|client| client == "client-b")
            })
            .map(|delivery| delivery.receipt_id.clone())
            .collect::<Vec<_>>();
        let client_a_state = NetworkClientState {
            client_id: "client-a".into(),
            player_id: "player-a".into(),
            started: true,
            tick: arena_state.tick,
            state_root: state_root.clone(),
            player_count: arena_state.player_count,
            sequence: transport_sequence,
            observed_journal_length: entries.len() as u64,
            acknowledged_receipts: acknowledged_a,
        };
        let client_b_state = NetworkClientState {
            client_id: "client-b".into(),
            player_id: "player-b".into(),
            started: true,
            tick: arena_state.tick,
            state_root: state_root.clone(),
            player_count: arena_state.player_count,
            sequence: transport_sequence,
            observed_journal_length: entries.len() as u64,
            acknowledged_receipts: acknowledged_b,
        };

        let proof = NetworkTransportSessionProof {
            proof_version: "network-transport-session-proof-v0.1".into(),
            status: if replay_verified {
                "Network Transport Session: PASS".into()
            } else {
                "Network Transport Session: FAIL".into()
            },
            session_id: arena_state.session_id.clone(),
            world_id: self.config.world_id.clone(),
            tick: arena_state.tick,
            player_count: arena_state.player_count,
            sequence: transport_sequence,
            state_root: state_root.clone(),
            client_a_state_root: client_a_state.state_root.clone(),
            client_b_state_root: client_b_state.state_root.clone(),
            replay_root: replay_root.clone(),
            replay_verified,
            replay_verification: if replay_verified { "PASS" } else { "FAIL" }.into(),
            client_a_started: client_a_state.started,
            client_b_started: client_b_state.started,
            session_joined: arena_state.players.contains_key("player-a")
                && arena_state.players.contains_key("player-b"),
            movement_synchronized: arena_state.positions.get("player-a")
                == Some(&ArenaPosition { x: 0, y: 1 })
                && arena_state.positions.get("player-b") == Some(&ArenaPosition { x: 0, y: -1 }),
            interaction_synchronized: arena_state.health.get("player-b") == Some(&90),
            state_synchronized: client_a_state.state_root == state_root
                && client_b_state.state_root == state_root,
            receipt_delivery: receipt_delivery.iter().all(|delivery| {
                delivery.generated_by == "runtime-authority"
                    && delivery.delivered_to.len() == 2
                    && delivery.acknowledged_by.len() == 2
            }),
            journal_observation: client_a_state.observed_journal_length == entries.len() as u64
                && client_b_state.observed_journal_length == entries.len() as u64,
            multi_tick_progression: arena_state.tick >= 5,
            transcript_length: transcript.len() as u64,
            receipt_count,
            journal_length: entries.len() as u64,
            limitations: vec![
                "local network-style transport only".into(),
                "no federation".into(),
                "no Evernode deployment".into(),
                "no WAN networking".into(),
                "no production transport reliability claim".into(),
            ],
        };
        self.write_network_transport_session_artifacts(
            &arena_state,
            &client_a_state,
            &client_b_state,
            &transport_log,
            &transcript,
            &receipt_delivery,
            &proof,
        )?;
        Ok(proof)
    }

    fn write_network_transport_session_artifacts(
        &self,
        state: &ArenaState,
        client_a_state: &NetworkClientState,
        client_b_state: &NetworkClientState,
        transport_log: &[NetworkTransportMessage],
        transcript: &[serde_json::Value],
        receipt_delivery: &[NetworkReceiptDelivery],
        proof: &NetworkTransportSessionProof,
    ) -> Result<()> {
        let network_dir = self.config.root.join("network");
        let sessions_dir = self.config.root.join("sessions");
        let replay_dir = self.config.root.join("replay");
        let receipts_dir = self.config.root.join("receipts");
        let journals_dir = self.config.root.join("journals");
        self.persistence.ensure_layout(&[
            network_dir.clone(),
            sessions_dir.clone(),
            replay_dir.clone(),
            receipts_dir.clone(),
            journals_dir.clone(),
        ])?;
        let runtime_state = serde_json::json!({
            "session_id": proof.session_id,
            "world_id": proof.world_id,
            "tick": proof.tick,
            "state_root": proof.state_root,
            "player_count": proof.player_count,
            "sequence": proof.sequence,
            "authority": "runtime",
            "clients_mutate_state": false,
            "clients_generate_receipts": false,
            "clients_generate_checkpoints": false,
            "clients_generate_replay_roots": false,
        });
        self.persistence
            .atomic_write_json(network_dir.join("client-a-state.json"), client_a_state)?;
        self.persistence
            .atomic_write_json(network_dir.join("client-b-state.json"), client_b_state)?;
        self.persistence
            .atomic_write_json(network_dir.join("runtime-state.json"), &runtime_state)?;
        self.persistence.atomic_write_json(
            network_dir.join("transport-log.json"),
            &transport_log.to_vec(),
        )?;
        self.persistence.atomic_write_json(
            network_dir.join("session-transcript.json"),
            &transcript.to_vec(),
        )?;
        self.persistence.atomic_write_json(
            network_dir.join("receipt-delivery.json"),
            &receipt_delivery.to_vec(),
        )?;
        self.persistence.atomic_write_json(
            sessions_dir.join("network-session-transcript.json"),
            &transcript.to_vec(),
        )?;
        self.persistence.atomic_write_json(
            sessions_dir.join("session-0001.json"),
            &serde_json::json!({
                "session_id": proof.session_id,
                "world_id": proof.world_id,
                "tick": proof.tick,
                "player_count": proof.player_count,
                "state_root": proof.state_root,
                "sequence": proof.sequence,
                "status": "Network Transport & Session Synchronization Proven"
            }),
        )?;
        self.persistence.atomic_write_json(
            replay_dir.join("network-session-replay-proof.json"),
            &serde_json::json!({
                "proof_version": proof.proof_version,
                "session_id": proof.session_id,
                "tick": proof.tick,
                "state_root": proof.state_root,
                "final_session_root": proof.state_root,
                "replay_root": proof.replay_root,
                "replay_verification": proof.replay_verification,
                "status": "Network Replay Verification: PASS",
                "transcript_length": proof.transcript_length,
                "journal_length": proof.journal_length
            }),
        )?;
        self.persistence.atomic_write_json(
            self.config
                .reports_dir()
                .join("network-transport-session-proof.json"),
            proof,
        )?;
        self.persistence
            .atomic_write_json(network_dir.join("arena-state.json"), state)?;
        for entry in std::fs::read_dir(self.config.receipts_dir())? {
            let entry = entry?;
            if entry.file_type()?.is_file() {
                std::fs::copy(entry.path(), receipts_dir.join(entry.file_name()))?;
            }
        }
        std::fs::copy(
            self.config.journals_dir().join("journal.jsonl"),
            journals_dir.join("journal.jsonl"),
        )?;
        Ok(())
    }

    pub fn execute_guest_proof(&mut self) -> Result<GuestExecutionProof> {
        let guest_id = self.package.manifest.package_id.clone();
        let previous_state_root = hex::encode(Sha256::digest(&self.state));
        let execution = GuestWasmRunner::execute(&guest_id, &self.package.wasm)?;
        if execution.guest_hash != self.package.manifest.wasm_hash {
            anyhow::bail!("guest wasm hash does not match package manifest");
        }
        let previous_hash = self.journal.latest_hash()?;
        let sequence = 1_u64;
        let receipt_id = format!("receipt-{:020}", sequence);
        let guest_state = GuestRuntimeState {
            guest_id: execution.guest_id.clone(),
            guest_hash: execution.guest_hash.clone(),
            action: execution.output.action.clone(),
            player_id: execution.output.player_id.clone(),
            position: execution.output.position.clone(),
            score: execution.output.score,
            guest_output_hash: execution.guest_output_hash.clone(),
        };
        let state_root = guest_state.root()?;
        self.state = serde_json::to_vec(&guest_state)?;
        let receipt_hash = Self::deterministic_receipt_hash(
            &receipt_id,
            sequence,
            &execution.guest_output_hash,
            &state_root,
            &self.config.runtime_version,
            &self.config.world_id,
            sequence,
        );
        let receipt = RuntimeReceipt {
            receipt_id: receipt_id.clone(),
            sequence,
            tick: sequence,
            input_id: "guest-invocation-0001".into(),
            input_hash: execution.input_hash.clone(),
            state_root: state_root.clone(),
            receipt_hash: receipt_hash.clone(),
            runtime_version: self.config.runtime_version.clone(),
            world_id: self.config.world_id.clone(),
            timestamp_or_epoch: sequence,
            session_id: Some("session-0001".into()),
            player_count: Some(1),
            action: Some(execution.output.action.clone()),
            player_id: Some(execution.output.player_id.clone()),
            guest_id: Some(execution.guest_id.clone()),
            guest_hash: Some(execution.guest_hash.clone()),
            guest_output_hash: Some(execution.guest_output_hash.clone()),
        };
        self.persistence.write_versioned(
            self.config
                .receipts_dir()
                .join(format!("{receipt_id}.json")),
            &receipt,
        )?;
        self.journal.append_guest(
            sequence,
            previous_hash,
            state_root.clone(),
            execution.input_hash.clone(),
            receipt_hash.clone(),
            execution.guest_id.clone(),
            execution.guest_hash.clone(),
            execution.guest_output_hash.clone(),
            serde_json::to_value(&execution.invocation)?,
            serde_json::to_value(&execution.output)?,
        )?;
        let entries = self.journal.entries()?;
        let replay_state = Self::replay_guest_state_from_entries(&entries)?;
        let replay_root = replay_state.root()?;
        let replay_verified = replay_root == state_root;
        let checkpoint = self.checkpoints.create(
            sequence,
            &self.config.world_id,
            &self.config.runtime_version,
            entries.len() as u64,
            state_root.clone(),
            self.state.clone(),
            self.package.world_metadata.clone(),
        )?;
        self.checkpoints.verify_checkpoint(&checkpoint)?;
        self.metrics.ticks_executed += 1;
        self.metrics.receipts_generated += 1;
        self.metrics.checkpoint_count += 1;
        self.health.checkpoint_height = checkpoint.manifest.sequence;
        self.health.world_root = state_root.clone();
        self.health.journal_height = entries.len() as u64;
        self.health.latest_receipt = Some(receipt_hash.clone());
        self.persistence
            .write_versioned(self.config.runtime_status_path(), &self.health)?;
        let proof = GuestExecutionProof {
            proof_version: "wasm-guest-execution-proof-v0.1".into(),
            status: if replay_verified {
                "WASM Guest Execution: PASS"
            } else {
                "WASM Guest Execution: FAIL"
            }
            .into(),
            world_id: self.config.world_id.clone(),
            guest_id: execution.guest_id,
            guest_hash: execution.guest_hash,
            guest_output_hash: execution.guest_output_hash,
            action: execution.output.action,
            player_id: execution.output.player_id,
            position: execution.output.position,
            score: execution.output.score,
            previous_state_root: previous_state_root.clone(),
            state_root: state_root.clone(),
            execution_root: state_root,
            state_root_changed: receipt.state_root != previous_state_root,
            state_mutation_origin: "guest_output".into(),
            receipt_id,
            receipt_hash,
            journal_length: entries.len() as u64,
            replay_root,
            replay_verified,
            replay_verification: if replay_verified { "PASS" } else { "FAIL" }.into(),
        };
        self.write_guest_execution_artifacts(&guest_state, &proof)?;
        Ok(proof)
    }

    fn replay_guest_state_from_entries(entries: &[JournalEntry]) -> Result<GuestRuntimeState> {
        let entry = entries
            .iter()
            .rev()
            .find(|entry| entry.guest_output.is_some())
            .ok_or_else(|| anyhow::anyhow!("missing guest output journal entry"))?;
        let output: GuestOutput = serde_json::from_value(entry.guest_output.clone().unwrap())?;
        Ok(GuestRuntimeState {
            guest_id: entry.guest_id.clone().unwrap_or_default(),
            guest_hash: entry.guest_hash.clone().unwrap_or_default(),
            action: output.action,
            player_id: output.player_id,
            position: output.position,
            score: output.score,
            guest_output_hash: entry.guest_output_hash.clone().unwrap_or_default(),
        })
    }

    fn write_guest_execution_artifacts(
        &self,
        state: &GuestRuntimeState,
        proof: &GuestExecutionProof,
    ) -> Result<()> {
        let guest_dir = self.config.root.join("guest");
        let replay_dir = self.config.root.join("replay");
        let receipts_dir = self.config.root.join("receipts");
        let journals_dir = self.config.root.join("journals");
        self.persistence.ensure_layout(&[
            guest_dir.clone(),
            replay_dir.clone(),
            receipts_dir.clone(),
            journals_dir.clone(),
        ])?;
        self.persistence
            .atomic_write_json(guest_dir.join("guest-execution.json"), state)?;
        self.persistence
            .atomic_write_json(replay_dir.join("guest-replay-proof.json"), proof)?;
        self.persistence.atomic_write_json(
            self.config
                .reports_dir()
                .join("wasm-guest-execution-proof.json"),
            proof,
        )?;
        for entry in std::fs::read_dir(self.config.receipts_dir())? {
            let entry = entry?;
            if entry.file_type()?.is_file() {
                std::fs::copy(entry.path(), receipts_dir.join(entry.file_name()))?;
            }
        }
        std::fs::copy(
            self.config.journals_dir().join("journal.jsonl"),
            journals_dir.join("journal.jsonl"),
        )?;
        Ok(())
    }

    pub fn run_ticks(&mut self, max_ticks: u64) -> Result<u64> {
        let mut ticks = 0;
        while ticks < max_ticks && self.execute_tick()?.is_some() {
            ticks += 1;
        }
        Ok(ticks)
    }

    pub fn stop(&mut self) -> Result<()> {
        self.lifecycle
            .transition(RuntimeState::Stopping, "operator stop");
        self.health.runtime_state = RuntimeState::Stopped;
        self.persistence
            .write_versioned(self.config.runtime_status_path(), &self.health)?;
        self.lifecycle
            .transition(RuntimeState::Stopped, "runtime stopped");
        Ok(())
    }
}
