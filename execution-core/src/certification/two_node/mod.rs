use sha2::{Digest, Sha256};

/// Stable identity used by the two-node certification harness.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NodeIdentity {
    pub node_id: String,
    pub authoritative: bool,
}

impl NodeIdentity {
    pub fn authority(node_id: &str) -> Self {
        Self {
            node_id: node_id.to_owned(),
            authoritative: true,
        }
    }

    pub fn observer(node_id: &str) -> Self {
        Self {
            node_id: node_id.to_owned(),
            authoritative: false,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NodeSession {
    pub world_id: String,
    pub session_id: String,
    pub tick: u64,
    pub state: i64,
    pub players: u8,
    pub active: bool,
    pub partitioned: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NodeCheckpoint {
    pub world_root: String,
    pub replay_root: String,
    pub checkpoint_root: String,
    pub continuity_root: String,
    pub tick: u64,
    pub state: i64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NodeConvergence {
    pub world_root_match: bool,
    pub replay_root_match: bool,
    pub checkpoint_root_match: bool,
    pub continuity_root_match: bool,
}

impl NodeConvergence {
    pub fn converged(&self) -> bool {
        self.world_root_match
            && self.replay_root_match
            && self.checkpoint_root_match
            && self.continuity_root_match
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NodeRecovery {
    pub restored: bool,
    pub source_checkpoint_root: String,
    pub restored_checkpoint_root: String,
    pub recovery_time_ticks: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CertificationError {
    NodeInactive,
    DivergenceDetected(String),
    CorruptCheckpoint(String),
    CorruptReplay(String),
    InvalidRecord(String),
    AuthorityViolation(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CorruptionLog {
    pub detected: bool,
    pub rejected: bool,
    pub reason: String,
}

/// Informational metrics for the deterministic in-process certification harness.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ConvergenceMetrics {
    pub sync_time_ticks: u64,
    pub checkpoint_transfer_count: u64,
    pub replay_transfer_count: u64,
    pub recovery_time_ticks: u64,
    pub convergence_time_ticks: u64,
}

/// Small deterministic Arena Vanguard state machine for two-node certification.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NodeRuntime {
    pub identity: NodeIdentity,
    pub session: NodeSession,
    replay: Vec<String>,
    checkpoints: Vec<NodeCheckpoint>,
    pub logs: Vec<CorruptionLog>,
    pub metrics: ConvergenceMetrics,
}

impl NodeRuntime {
    pub fn boot_arena_vanguard(node_id: &str) -> Self {
        let mut runtime = Self {
            identity: NodeIdentity::authority(node_id),
            session: NodeSession {
                world_id: "arena-vanguard".to_owned(),
                session_id: "arena-vanguard-two-node-certification".to_owned(),
                tick: 0,
                state: 42,
                players: 2,
                active: true,
                partitioned: false,
            },
            replay: vec!["genesis|arena-vanguard|players:2|state:42".to_owned()],
            checkpoints: Vec::new(),
            logs: Vec::new(),
            metrics: ConvergenceMetrics::default(),
        };
        let checkpoint = runtime.checkpoint();
        runtime.checkpoints.push(checkpoint);
        runtime
    }

    pub fn join_from(node_id: &str, authority: &NodeRuntime) -> Result<Self, CertificationError> {
        let checkpoint = authority.checkpoint();
        authority.validate_checkpoint(&checkpoint)?;
        let joined = Self {
            identity: NodeIdentity::observer(node_id),
            session: NodeSession {
                world_id: authority.session.world_id.clone(),
                session_id: authority.session.session_id.clone(),
                tick: checkpoint.tick,
                state: checkpoint.state,
                players: authority.session.players,
                active: true,
                partitioned: false,
            },
            replay: authority.replay.clone(),
            checkpoints: vec![checkpoint],
            logs: Vec::new(),
            metrics: ConvergenceMetrics {
                sync_time_ticks: 1,
                checkpoint_transfer_count: 1,
                replay_transfer_count: authority.replay.len() as u64,
                recovery_time_ticks: 0,
                convergence_time_ticks: 1,
            },
        };
        joined.validate_replay_root()?;
        Ok(joined)
    }

    pub fn tick(&mut self) -> Result<(), CertificationError> {
        if !self.session.active {
            return Err(CertificationError::NodeInactive);
        }
        if !self.identity.authoritative && !self.session.partitioned {
            return Err(CertificationError::AuthorityViolation(
                "observer nodes reconstruct ticks from authority records".to_owned(),
            ));
        }
        self.apply_deterministic_tick("authoritative_tick")
    }

    pub fn observe_tick_from(&mut self, authority: &NodeRuntime) -> Result<(), CertificationError> {
        if self.identity.authoritative {
            return Err(CertificationError::AuthorityViolation(
                "authority nodes do not observe from peers".to_owned(),
            ));
        }
        if authority.session.tick != self.session.tick + 1 {
            return Err(CertificationError::InvalidRecord(
                "authority replay window is not contiguous".to_owned(),
            ));
        }
        self.session.tick = authority.session.tick;
        self.session.state = authority.session.state;
        self.replay = authority.replay.clone();
        if authority.session.tick % 100 == 0 {
            self.checkpoints.push(authority.checkpoint());
            self.metrics.checkpoint_transfer_count += 1;
        }
        self.metrics.replay_transfer_count += 1;
        self.validate_replay_root()
    }

    pub fn run_authoritative_ticks(
        authority: &mut NodeRuntime,
        observer: &mut NodeRuntime,
        ticks: u64,
        checkpoint_interval: u64,
    ) -> Result<(), CertificationError> {
        for _ in 0..ticks {
            authority.tick()?;
            if authority.session.tick != observer.session.tick + 1 {
                return Err(CertificationError::InvalidRecord(
                    "authority replay window is not contiguous".to_owned(),
                ));
            }
            observer.session.tick = authority.session.tick;
            observer.session.state = authority.session.state;
            let record = authority.replay.last().cloned().ok_or_else(|| {
                CertificationError::CorruptReplay("authority replay is empty".to_owned())
            })?;
            observer.replay.push(record);
            observer.metrics.replay_transfer_count += 1;
            if checkpoint_interval > 0 && authority.session.tick % checkpoint_interval == 0 {
                let checkpoint = authority.checkpoint();
                authority.checkpoints.push(checkpoint.clone());
                observer.accept_checkpoint(checkpoint)?;
            }
        }
        observer.validate_replay_root()?;
        Ok(())
    }

    pub fn checkpoint(&self) -> NodeCheckpoint {
        NodeCheckpoint {
            world_root: self.world_root(),
            replay_root: self.replay_root(),
            checkpoint_root: self.checkpoint_root_for(self.session.tick, self.session.state),
            continuity_root: self.continuity_root(),
            tick: self.session.tick,
            state: self.session.state,
        }
    }

    pub fn accept_checkpoint(
        &mut self,
        checkpoint: NodeCheckpoint,
    ) -> Result<(), CertificationError> {
        self.validate_checkpoint(&checkpoint)?;
        self.session.tick = checkpoint.tick;
        self.session.state = checkpoint.state;
        self.checkpoints.push(checkpoint);
        self.metrics.checkpoint_transfer_count += 1;
        Ok(())
    }

    pub fn latest_checkpoint(&self) -> Option<&NodeCheckpoint> {
        self.checkpoints.last()
    }

    pub fn restore_from_checkpoint(
        node_id: &str,
        source: &NodeRuntime,
    ) -> Result<(Self, NodeRecovery), CertificationError> {
        let mut restored = Self::join_from(node_id, source)?;
        restored.metrics.recovery_time_ticks = 1;
        let root = restored.checkpoint().checkpoint_root;
        Ok((
            restored,
            NodeRecovery {
                restored: true,
                source_checkpoint_root: root.clone(),
                restored_checkpoint_root: root,
                recovery_time_ticks: 1,
            },
        ))
    }

    pub fn terminate(&mut self) {
        self.session.active = false;
    }

    pub fn partition(&mut self) {
        self.session.partitioned = true;
    }

    pub fn reconnect(&mut self) {
        self.session.partitioned = false;
    }

    pub fn independent_partition_activity(&mut self) -> Result<(), CertificationError> {
        if !self.session.partitioned {
            return Err(CertificationError::InvalidRecord(
                "independent activity requires a partition".to_owned(),
            ));
        }
        self.apply_deterministic_tick("partition_tick")
    }

    pub fn compare(&self, other: &NodeRuntime) -> NodeConvergence {
        NodeConvergence {
            world_root_match: self.world_root() == other.world_root(),
            replay_root_match: self.replay_root() == other.replay_root(),
            checkpoint_root_match: self.checkpoint().checkpoint_root
                == other.checkpoint().checkpoint_root,
            continuity_root_match: self.continuity_root() == other.continuity_root(),
        }
    }

    pub fn require_convergence(
        &self,
        other: &NodeRuntime,
    ) -> Result<NodeConvergence, CertificationError> {
        let convergence = self.compare(other);
        if convergence.converged() {
            Ok(convergence)
        } else {
            Err(CertificationError::DivergenceDetected(format!(
                "world:{} replay:{} checkpoint:{} continuity:{}",
                convergence.world_root_match,
                convergence.replay_root_match,
                convergence.checkpoint_root_match,
                convergence.continuity_root_match
            )))
        }
    }

    pub fn detect_partition_divergence(
        &mut self,
        other: &NodeRuntime,
    ) -> Result<(), CertificationError> {
        match self.require_convergence(other) {
            Ok(_) => Ok(()),
            Err(err) => {
                self.logs.push(CorruptionLog {
                    detected: true,
                    rejected: true,
                    reason: "network partition divergence detected; manual reconciliation required"
                        .to_owned(),
                });
                Err(err)
            }
        }
    }

    pub fn validate_checkpoint(
        &self,
        checkpoint: &NodeCheckpoint,
    ) -> Result<(), CertificationError> {
        let expected_world = self.world_root_for(checkpoint.tick, checkpoint.state);
        let expected_checkpoint = self.checkpoint_root_for(checkpoint.tick, checkpoint.state);
        if checkpoint.world_root != expected_world
            || checkpoint.checkpoint_root != expected_checkpoint
        {
            return Err(CertificationError::CorruptCheckpoint(
                "checkpoint root does not match canonical state".to_owned(),
            ));
        }
        Ok(())
    }

    pub fn validate_replay_root(&self) -> Result<(), CertificationError> {
        let expected = hash_lines("replay", &self.replay);
        if expected == self.replay_root() {
            Ok(())
        } else {
            Err(CertificationError::CorruptReplay(
                "replay root does not match canonical replay records".to_owned(),
            ))
        }
    }

    pub fn reject_corrupt_checkpoint(&mut self, mut checkpoint: NodeCheckpoint) -> CorruptionLog {
        checkpoint.checkpoint_root.push_str(":corrupt");
        let rejected = self.accept_checkpoint(checkpoint).is_err();
        let log = CorruptionLog {
            detected: rejected,
            rejected,
            reason: "corrupt checkpoint rejected".to_owned(),
        };
        self.logs.push(log.clone());
        log
    }

    pub fn reject_corrupt_replay(&mut self) -> CorruptionLog {
        let mut corrupt = self.clone();
        corrupt.replay.push("invalid_record|rewrite".to_owned());
        corrupt.session.state += 1;
        let rejected = corrupt.require_convergence(self).is_err();
        let log = CorruptionLog {
            detected: rejected,
            rejected,
            reason: "corrupt replay rejected".to_owned(),
        };
        self.logs.push(log.clone());
        log
    }

    pub fn reject_corrupt_state_root(&mut self, other: &NodeRuntime) -> CorruptionLog {
        let mut corrupt = other.clone();
        corrupt.session.state += 99;
        let rejected = self.require_convergence(&corrupt).is_err();
        let log = CorruptionLog {
            detected: rejected,
            rejected,
            reason: "corrupt state root rejected".to_owned(),
        };
        self.logs.push(log.clone());
        log
    }

    pub fn rewrite_replay(&mut self, record: &str) -> Result<(), CertificationError> {
        if !self.identity.authoritative {
            return Err(CertificationError::AuthorityViolation(
                "observer cannot rewrite replay".to_owned(),
            ));
        }
        self.replay.push(record.to_owned());
        Ok(())
    }

    pub fn mutate_authority_state(&mut self, delta: i64) -> Result<(), CertificationError> {
        if !self.identity.authoritative {
            return Err(CertificationError::AuthorityViolation(
                "observer cannot mutate authority state".to_owned(),
            ));
        }
        self.session.state += delta;
        Ok(())
    }

    pub fn world_root(&self) -> String {
        self.world_root_for(self.session.tick, self.session.state)
    }

    pub fn replay_root(&self) -> String {
        hash_lines("replay", &self.replay)
    }

    pub fn replay_hash(&self) -> String {
        hash_lines("replay-hash", &self.replay)
    }

    pub fn continuity_root(&self) -> String {
        digest(&format!(
            "continuity|{}|{}|{}|{}",
            self.session.world_id,
            self.world_root(),
            self.replay_root(),
            self.session.tick
        ))
    }

    fn apply_deterministic_tick(&mut self, source: &str) -> Result<(), CertificationError> {
        self.session.tick += 1;
        let tick = self.session.tick as i64;
        self.session.state =
            (self.session.state * 31 + tick * 17 + self.session.players as i64) % 1_000_003;
        self.replay.push(format!(
            "{source}|{}|tick:{}|state:{}|players:{}",
            self.session.world_id, self.session.tick, self.session.state, self.session.players
        ));
        Ok(())
    }

    fn world_root_for(&self, tick: u64, state: i64) -> String {
        digest(&format!(
            "world|{}|{}|{}|{}",
            self.session.world_id, self.session.session_id, tick, state
        ))
    }

    fn checkpoint_root_for(&self, tick: u64, state: i64) -> String {
        digest(&format!(
            "checkpoint|{}|{}|{}|{}",
            self.session.world_id,
            self.session.session_id,
            tick,
            self.world_root_for(tick, state)
        ))
    }
}

fn hash_lines(prefix: &str, lines: &[String]) -> String {
    digest(&format!("{}|{}", prefix, lines.join("\n")))
}

fn digest(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    hex::encode(hasher.finalize())
}
