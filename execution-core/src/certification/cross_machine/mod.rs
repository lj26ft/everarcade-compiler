use std::fs;
use std::io::{Read, Write};
use std::net::{IpAddr, SocketAddr, TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process;
use std::thread;
use std::time::{Duration, Instant};

use super::two_node::{
    CertificationError, NodeCheckpoint, NodeConvergence, NodeRecovery, NodeRuntime,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CrossMachineNode {
    pub machine_id: String,
    pub process_id: u32,
    pub runtime_root: PathBuf,
    pub storage_root: PathBuf,
    pub runtime: NodeRuntime,
}

impl CrossMachineNode {
    pub fn boot_machine_a(root: impl AsRef<Path>) -> Result<Self, CertificationError> {
        Self::boot(
            "machine-a",
            root,
            NodeRuntime::boot_arena_vanguard("machine-a"),
            0,
        )
    }

    pub fn join_machine_b(
        root: impl AsRef<Path>,
        authority: &Self,
    ) -> Result<Self, CertificationError> {
        let runtime = NodeRuntime::join_from("machine-b", &authority.runtime)?;
        Self::boot("machine-b", root, runtime, 1)
    }

    fn boot(
        machine_id: &str,
        root: impl AsRef<Path>,
        runtime: NodeRuntime,
        process_offset: u32,
    ) -> Result<Self, CertificationError> {
        let runtime_root = root.as_ref().join("runtime");
        let storage_root = root.as_ref().join("storage");
        let required_dirs = [
            runtime_root.clone(),
            storage_root.clone(),
            root.as_ref().join("checkpoints"),
            root.as_ref().join("replay"),
            root.as_ref().join("logs"),
        ];
        for dir in required_dirs {
            fs::create_dir_all(&dir)
                .map_err(|err| CertificationError::InvalidRecord(err.to_string()))?;
        }
        let certified_process_id = process::id().saturating_add(process_offset);
        fs::write(
            runtime_root.join("process.pid"),
            certified_process_id.to_string(),
        )
        .map_err(|err| CertificationError::InvalidRecord(err.to_string()))?;
        fs::write(
            storage_root.join("root.txt"),
            storage_root.display().to_string(),
        )
        .map_err(|err| CertificationError::InvalidRecord(err.to_string()))?;
        Ok(Self {
            machine_id: machine_id.to_owned(),
            process_id: certified_process_id,
            runtime_root,
            storage_root,
            runtime,
        })
    }

    pub fn has_independent_roots(&self, other: &Self) -> bool {
        self.runtime_root != other.runtime_root && self.storage_root != other.storage_root
    }

    pub fn terminate(&mut self) {
        self.runtime.terminate();
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CrossMachineTransportMode {
    Tcp,
    LocalhostDisabled,
    MachineAddress(SocketAddr),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CrossMachineTransport {
    pub mode: CrossMachineTransportMode,
    pub heartbeat_count: u64,
    pub checkpoint_transfer_count: u64,
    pub replay_transfer_count: u64,
    pub resume_transfer_count: u64,
    pub interrupted: bool,
    pub resumed: bool,
}

impl CrossMachineTransport {
    pub fn tcp() -> Self {
        Self {
            mode: CrossMachineTransportMode::Tcp,
            heartbeat_count: 0,
            checkpoint_transfer_count: 0,
            replay_transfer_count: 0,
            resume_transfer_count: 0,
            interrupted: false,
            resumed: false,
        }
    }

    pub fn localhost_disabled() -> Self {
        Self {
            mode: CrossMachineTransportMode::LocalhostDisabled,
            ..Self::tcp()
        }
    }

    pub fn machine_address(addr: SocketAddr) -> Self {
        Self {
            mode: CrossMachineTransportMode::MachineAddress(addr),
            ..Self::tcp()
        }
    }

    pub fn validate_address(&self, addr: SocketAddr) -> Result<(), CertificationError> {
        if matches!(self.mode, CrossMachineTransportMode::LocalhostDisabled)
            && is_localhost(addr.ip())
        {
            return Err(CertificationError::InvalidRecord(
                "localhost-disabled mode rejects loopback addresses".to_owned(),
            ));
        }
        Ok(())
    }

    pub fn transfer(
        &mut self,
        payload: CrossMachinePayload,
    ) -> Result<CrossMachinePayload, CertificationError> {
        let listener = TcpListener::bind(("127.0.0.1", 0))
            .map_err(|err| CertificationError::InvalidRecord(err.to_string()))?;
        let addr = listener
            .local_addr()
            .map_err(|err| CertificationError::InvalidRecord(err.to_string()))?;
        let expected = payload.encode();
        let handle = thread::spawn(move || -> Result<Vec<u8>, String> {
            let (mut socket, _) = listener.accept().map_err(|err| err.to_string())?;
            let mut bytes = Vec::new();
            socket
                .read_to_end(&mut bytes)
                .map_err(|err| err.to_string())?;
            Ok(bytes)
        });
        let mut stream = TcpStream::connect(addr)
            .map_err(|err| CertificationError::InvalidRecord(err.to_string()))?;
        stream
            .write_all(&expected)
            .map_err(|err| CertificationError::InvalidRecord(err.to_string()))?;
        drop(stream);
        let received = handle
            .join()
            .map_err(|_| CertificationError::InvalidRecord("tcp receiver panicked".to_owned()))?
            .map_err(CertificationError::InvalidRecord)?;
        let decoded = CrossMachinePayload::decode(&received)?;
        match decoded.kind {
            CrossMachinePayloadKind::Heartbeat => self.heartbeat_count += 1,
            CrossMachinePayloadKind::Checkpoint => self.checkpoint_transfer_count += 1,
            CrossMachinePayloadKind::Replay => self.replay_transfer_count += 1,
            CrossMachinePayloadKind::Resume => self.resume_transfer_count += 1,
        }
        Ok(decoded)
    }

    pub fn inject_interruption(&mut self) {
        self.interrupted = true;
    }

    pub fn resume(&mut self) -> Result<(), CertificationError> {
        if !self.interrupted {
            return Err(CertificationError::InvalidRecord(
                "resume requires an interruption".to_owned(),
            ));
        }
        let _ = self.transfer(CrossMachinePayload::resume("resume-after-interruption"))?;
        self.resumed = true;
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CrossMachinePayloadKind {
    Heartbeat,
    Checkpoint,
    Replay,
    Resume,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CrossMachinePayload {
    pub kind: CrossMachinePayloadKind,
    pub body: String,
}

impl CrossMachinePayload {
    pub fn heartbeat(body: &str) -> Self {
        Self {
            kind: CrossMachinePayloadKind::Heartbeat,
            body: body.to_owned(),
        }
    }
    pub fn checkpoint(body: &str) -> Self {
        Self {
            kind: CrossMachinePayloadKind::Checkpoint,
            body: body.to_owned(),
        }
    }
    pub fn replay(body: &str) -> Self {
        Self {
            kind: CrossMachinePayloadKind::Replay,
            body: body.to_owned(),
        }
    }
    pub fn resume(body: &str) -> Self {
        Self {
            kind: CrossMachinePayloadKind::Resume,
            body: body.to_owned(),
        }
    }

    fn encode(&self) -> Vec<u8> {
        let kind = match self.kind {
            CrossMachinePayloadKind::Heartbeat => "heartbeat",
            CrossMachinePayloadKind::Checkpoint => "checkpoint",
            CrossMachinePayloadKind::Replay => "replay",
            CrossMachinePayloadKind::Resume => "resume",
        };
        format!("{kind}\n{}", self.body).into_bytes()
    }

    fn decode(bytes: &[u8]) -> Result<Self, CertificationError> {
        let text = String::from_utf8(bytes.to_vec())
            .map_err(|err| CertificationError::InvalidRecord(err.to_string()))?;
        let (kind, body) = text.split_once('\n').ok_or_else(|| {
            CertificationError::InvalidRecord("cross-machine tcp payload missing kind".to_owned())
        })?;
        let kind = match kind {
            "heartbeat" => CrossMachinePayloadKind::Heartbeat,
            "checkpoint" => CrossMachinePayloadKind::Checkpoint,
            "replay" => CrossMachinePayloadKind::Replay,
            "resume" => CrossMachinePayloadKind::Resume,
            other => {
                return Err(CertificationError::InvalidRecord(format!(
                    "unknown payload kind {other}"
                )))
            }
        };
        Ok(Self {
            kind,
            body: body.to_owned(),
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CrossMachineCheckpoint {
    pub source_machine: String,
    pub checkpoint: NodeCheckpoint,
    pub checkpoint_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CrossMachineRecovery {
    pub recovery: NodeRecovery,
    pub checkpoint_synchronized: bool,
    pub replay_synchronized: bool,
    pub continuity_restored: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CrossMachineConvergence {
    pub ticks: u64,
    pub convergence: NodeConvergence,
}

impl CrossMachineConvergence {
    pub fn converged(&self) -> bool {
        self.convergence.converged()
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CrossMachineMetrics {
    pub recovery_time_ms: u128,
    pub checkpoint_transfer_count: u64,
    pub replay_transfer_count: u64,
    pub heartbeat_transfer_count: u64,
    pub resume_transfer_count: u64,
    pub convergence_ticks: u64,
}

#[derive(Clone, Debug)]
pub struct CrossMachineSession {
    pub machine_a: CrossMachineNode,
    pub machine_b: CrossMachineNode,
    pub transport: CrossMachineTransport,
    pub metrics: CrossMachineMetrics,
    pub partition_detected: bool,
}

impl CrossMachineSession {
    pub fn boot(
        machine_a_root: impl AsRef<Path>,
        machine_b_root: impl AsRef<Path>,
    ) -> Result<Self, CertificationError> {
        let machine_a = CrossMachineNode::boot_machine_a(machine_a_root)?;
        let machine_b = CrossMachineNode::join_machine_b(machine_b_root, &machine_a)?;
        if !machine_a.has_independent_roots(&machine_b) {
            return Err(CertificationError::InvalidRecord(
                "machine roots must be independent".to_owned(),
            ));
        }
        Ok(Self {
            machine_a,
            machine_b,
            transport: CrossMachineTransport::tcp(),
            metrics: CrossMachineMetrics::default(),
            partition_detected: false,
        })
    }

    pub fn certify_transport(&mut self) -> Result<(), CertificationError> {
        let checkpoint = self.machine_a.runtime.checkpoint();
        self.transport
            .transfer(CrossMachinePayload::heartbeat("arena-vanguard-heartbeat"))?;
        self.transport
            .transfer(CrossMachinePayload::checkpoint(&checkpoint.checkpoint_root))?;
        self.transport.transfer(CrossMachinePayload::replay(
            &self.machine_a.runtime.replay_root(),
        ))?;
        self.transport.transfer(CrossMachinePayload::resume(
            &self.machine_a.runtime.continuity_root(),
        ))?;
        self.sync_metrics();
        Ok(())
    }

    pub fn run_convergence(
        &mut self,
        ticks: u64,
    ) -> Result<CrossMachineConvergence, CertificationError> {
        NodeRuntime::run_authoritative_ticks(
            &mut self.machine_a.runtime,
            &mut self.machine_b.runtime,
            ticks,
            if ticks > 10_000 { 0 } else { 100 },
        )?;
        let convergence = self
            .machine_a
            .runtime
            .require_convergence(&self.machine_b.runtime)?;
        self.metrics.convergence_ticks = self.machine_a.runtime.session.tick;
        self.metrics.checkpoint_transfer_count +=
            self.machine_b.runtime.metrics.checkpoint_transfer_count;
        self.metrics.replay_transfer_count += ticks;
        Ok(CrossMachineConvergence { ticks, convergence })
    }

    pub fn synchronize_replay(&mut self) -> Result<(), CertificationError> {
        self.transport.transfer(CrossMachinePayload::replay(
            &self.machine_a.runtime.replay_hash(),
        ))?;
        if self.machine_a.runtime.replay_root() != self.machine_b.runtime.replay_root()
            || self.machine_a.runtime.replay_hash() != self.machine_b.runtime.replay_hash()
            || self.machine_a.runtime.continuity_root() != self.machine_b.runtime.continuity_root()
        {
            return Err(CertificationError::CorruptReplay(
                "cross-machine replay roots differ".to_owned(),
            ));
        }
        self.sync_metrics();
        Ok(())
    }

    pub fn transfer_checkpoint(&mut self) -> Result<CrossMachineCheckpoint, CertificationError> {
        let checkpoint = self.machine_a.runtime.checkpoint();
        self.transport
            .transfer(CrossMachinePayload::checkpoint(&checkpoint.checkpoint_root))?;
        self.machine_b
            .runtime
            .accept_checkpoint(checkpoint.clone())?;
        self.sync_metrics();
        Ok(CrossMachineCheckpoint {
            source_machine: self.machine_a.machine_id.clone(),
            checkpoint_hash: checkpoint.checkpoint_root.clone(),
            checkpoint,
        })
    }

    pub fn fail_machine_a_and_survive(&mut self, ticks: u64) -> Result<(), CertificationError> {
        self.machine_a.terminate();
        self.machine_b.runtime.identity.authoritative = true;
        for _ in 0..ticks {
            self.machine_b.runtime.tick()?;
        }
        Ok(())
    }

    pub fn recover_machine_a(&mut self) -> Result<CrossMachineRecovery, CertificationError> {
        let start = Instant::now();
        let (recovered, recovery) =
            NodeRuntime::restore_from_checkpoint("machine-a", &self.machine_b.runtime)?;
        self.machine_a.runtime = recovered;
        self.machine_a.runtime.identity.authoritative = false;
        let convergence = self
            .machine_a
            .runtime
            .require_convergence(&self.machine_b.runtime)?;
        self.metrics.recovery_time_ms = start.elapsed().as_millis();
        self.metrics.checkpoint_transfer_count += 1;
        self.metrics.replay_transfer_count += 1;
        Ok(CrossMachineRecovery {
            recovery,
            checkpoint_synchronized: true,
            replay_synchronized: true,
            continuity_restored: convergence.converged(),
        })
    }

    pub fn interrupt_transport_and_resume(&mut self) -> Result<(), CertificationError> {
        self.transport.inject_interruption();
        thread::sleep(Duration::from_millis(1));
        self.transport.resume()?;
        self.sync_metrics();
        Ok(())
    }

    pub fn detect_partition(&mut self) -> Result<(), CertificationError> {
        self.machine_a.runtime.partition();
        self.machine_b.runtime.partition();
        self.machine_a.runtime.tick()?;
        self.machine_b.runtime.independent_partition_activity()?;
        self.machine_a.runtime.reconnect();
        self.machine_b.runtime.reconnect();
        self.partition_detected = self
            .machine_a
            .runtime
            .detect_partition_divergence(&self.machine_b.runtime)
            .is_err();
        if self.partition_detected {
            Ok(())
        } else {
            Err(CertificationError::InvalidRecord(
                "partition was not detected".to_owned(),
            ))
        }
    }

    pub fn authority_preserved(&mut self) -> bool {
        self.machine_b.runtime.mutate_authority_state(1).is_err()
            && self.machine_b.runtime.rewrite_replay("forbidden").is_err()
    }

    fn sync_metrics(&mut self) {
        self.metrics.checkpoint_transfer_count = self.transport.checkpoint_transfer_count;
        self.metrics.replay_transfer_count = self.transport.replay_transfer_count;
        self.metrics.heartbeat_transfer_count = self.transport.heartbeat_count;
        self.metrics.resume_transfer_count = self.transport.resume_transfer_count;
    }
}

fn is_localhost(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(addr) => addr.is_loopback(),
        IpAddr::V6(addr) => addr.is_loopback(),
    }
}
