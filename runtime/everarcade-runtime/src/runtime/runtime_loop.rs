use crate::runtime::*;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RuntimeReceipt {
    pub sequence: u64,
    pub input_id: String,
    pub state_root: String,
    pub receipt_hash: String,
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
            config.backups_dir(),
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
        Ok(Self {
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
        })
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
        self.state.extend_from_slice(&input.payload);
        self.state.extend_from_slice(input.payload_hash.as_bytes());
        let state_root = hex::encode(Sha256::digest(&self.state));
        let receipt_hash = hex::encode(Sha256::digest(
            format!("{}:{}:{}", input.sequence, input.input_id, state_root).as_bytes(),
        ));
        let receipt = RuntimeReceipt {
            sequence: input.sequence,
            input_id: input.input_id.clone(),
            state_root: state_root.clone(),
            receipt_hash: receipt_hash.clone(),
        };
        self.persistence.write_versioned(
            self.config
                .receipts_dir()
                .join(format!("receipt-{:020}.json", input.sequence)),
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
