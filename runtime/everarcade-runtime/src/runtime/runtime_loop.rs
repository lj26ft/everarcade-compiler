use crate::runtime::*;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
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
