#[path = "../../runtime/renderer-client/src/transport_runtime/mod.rs"]
mod transport_runtime;

use execution_core::certification::two_node::{NodeIdentity, NodeRuntime};
use std::fs::{self, OpenOptions};
use std::io::{BufReader, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};
use tempfile::TempDir;
use transport_runtime::peer_handshake::ReplayPeerHandshake;
use transport_runtime::resume::ReplayResumeRequest;
use transport_runtime::tcp::protocol::TcpReplayFrame;
use transport_runtime::tcp::session::TcpReplaySession;
use transport_runtime::tcp::stream::TcpReplayStream;
use transport_runtime::wire::{
    deterministic_hash, ReplayCheckpointWireMessage, ReplayChunkWireMessage, ReplayTransportError,
};
use transport_runtime::ReplayCursor;

const PROTOCOL: &str = "everarcade-replay/1";
const CONTINUITY_ROOT: &str = "arena-vanguard-real-transport-root";
const FIRST_PHASE_READY_TICK: u64 = 4;
const RECOVERY_TRANSFER_TICKS: u64 = 4;

#[test]
fn test_arena_vanguard_real_tcp_process_recovery_certification() {
    let tmp = TempDir::new().unwrap();
    let node_a_storage = tmp.path().join("machine-a-node-a-storage");
    let node_b_storage = tmp.path().join("machine-b-node-b-storage");
    fs::create_dir_all(&node_a_storage).unwrap();
    fs::create_dir_all(&node_b_storage).unwrap();

    let listener = TcpListener::bind(("127.0.0.1", 0)).unwrap();
    let addr = listener.local_addr().unwrap();
    drop(listener);

    let mut node_b = spawn_child("node-b", addr, &node_b_storage);
    wait_for_file(&node_b_storage.join("ready.addr"), Duration::from_secs(5));

    let mut node_a = spawn_child("node-a-primary", addr, &node_a_storage);
    wait_for_tick(
        &node_b_storage.join("progress.tick"),
        FIRST_PHASE_READY_TICK,
    );

    node_a.kill().unwrap();
    let _ = node_a.wait();
    let disconnected_tick = read_tick(&node_b_storage.join("progress.tick"));
    wait_for_tick(
        &node_b_storage.join("progress.tick"),
        disconnected_tick + RECOVERY_TRANSFER_TICKS,
    );
    let continued_tick = read_tick(&node_b_storage.join("progress.tick"));
    assert!(
        continued_tick > disconnected_tick,
        "node-b continued after node-a died"
    );

    let replay_before_recovery = fs::read_to_string(node_b_storage.join("replay.log")).unwrap();
    assert!(replay_before_recovery.contains("authoritative_tick"));
    assert!(latest_checkpoint_file(&node_b_storage).exists());

    let mut node_a_recovered = spawn_child("node-a-recover", addr, &node_a_storage);
    assert!(node_a_recovered.wait().unwrap().success());
    assert!(node_b.wait().unwrap().success());

    let proof = fs::read_to_string(node_a_storage.join("recovery-proof.txt")).unwrap();
    assert!(proof.contains("checkpoint_transfer=true"));
    assert!(proof.contains("replay_synchronization=true"));
    assert!(proof.contains("convergence=true"));
    assert!(proof.contains("continuity_preserved=true"));
    assert_ne!(
        fs::canonicalize(&node_a_storage).unwrap(),
        fs::canonicalize(&node_b_storage).unwrap()
    );
    assert!(
        fs::read_to_string(node_a_storage.join("process.pid")).unwrap()
            != fs::read_to_string(node_b_storage.join("process.pid")).unwrap()
    );
}

#[test]
#[ignore]
fn real_transport_child_runtime() {
    let role = std::env::var("EVERARCADE_REAL_TRANSPORT_ROLE").unwrap_or_default();
    if role.is_empty() {
        return;
    }
    let addr: SocketAddr = std::env::var("EVERARCADE_REAL_TRANSPORT_ADDR")
        .unwrap()
        .parse()
        .unwrap();
    let storage = PathBuf::from(std::env::var("EVERARCADE_REAL_TRANSPORT_STORAGE").unwrap());
    fs::create_dir_all(storage.join("checkpoints")).unwrap();
    fs::write(storage.join("process.pid"), std::process::id().to_string()).unwrap();
    match role.as_str() {
        "node-b" => run_node_b(addr, &storage),
        "node-a-primary" => run_node_a_primary(addr, &storage),
        "node-a-recover" => run_node_a_recover(addr, &storage),
        other => panic!("unknown real transport child role: {other}"),
    }
}

fn spawn_child(role: &str, addr: SocketAddr, storage: &Path) -> Child {
    Command::new(std::env::current_exe().unwrap())
        .arg("--exact")
        .arg("real_transport_child_runtime")
        .arg("--ignored")
        .arg("--nocapture")
        .env("EVERARCADE_REAL_TRANSPORT_ROLE", role)
        .env("EVERARCADE_REAL_TRANSPORT_ADDR", addr.to_string())
        .env("EVERARCADE_REAL_TRANSPORT_STORAGE", storage)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap()
}

fn run_node_a_primary(addr: SocketAddr, storage: &Path) {
    let mut runtime = NodeRuntime::boot_arena_vanguard("node-a");
    let mut stream = connect_with_handshake(addr, handshake("node-a", 0));
    let mut sequence = 0;
    send_chunk(
        &mut stream,
        storage,
        sequence,
        "genesis|arena-vanguard|players:2|state:42",
    );
    sequence += 1;
    loop {
        runtime.tick().unwrap();
        let record = format!(
            "authoritative_tick|tick:{}|state:{}|world:{}",
            runtime.session.tick, runtime.session.state, runtime.session.world_id
        );
        send_chunk(&mut stream, storage, sequence, &record);
        if runtime.session.tick % 2 == 0 {
            send_checkpoint(
                &mut stream,
                storage,
                sequence,
                runtime.checkpoint().checkpoint_root,
            );
        }
        sequence += 1;
        thread::sleep(Duration::from_millis(35));
    }
}

fn run_node_b(addr: SocketAddr, storage: &Path) {
    let listener = TcpListener::bind(addr).unwrap();
    fs::write(
        storage.join("ready.addr"),
        listener.local_addr().unwrap().to_string(),
    )
    .unwrap();
    let mut runtime = NodeRuntime::boot_arena_vanguard("node-b");
    runtime.identity = NodeIdentity::observer("node-b");
    let mut accepted_chunks = Vec::new();

    let (first_stream, _) = listener.accept().unwrap();
    accept_primary_connection(first_stream, &mut runtime, storage, &mut accepted_chunks);

    runtime.identity = NodeIdentity::authority("node-b");
    for _ in 0..RECOVERY_TRANSFER_TICKS {
        runtime.tick().unwrap();
        let sequence = accepted_chunks.len() as u64;
        let record = format!(
            "authoritative_tick|failover:node-b|tick:{}|state:{}|world:{}",
            runtime.session.tick, runtime.session.state, runtime.session.world_id
        );
        accepted_chunks.push(make_chunk(sequence, &record));
        persist_replay(storage, sequence, &record);
        write_tick(storage, runtime.session.tick);
        persist_checkpoint(
            storage,
            accepted_chunks.len() as u64,
            runtime.checkpoint().checkpoint_root,
        );
        thread::sleep(Duration::from_millis(45));
    }

    let (recovery_stream, _) = listener.accept().unwrap();
    serve_recovery_connection(recovery_stream, &runtime, storage, &accepted_chunks);
}

fn run_node_a_recover(addr: SocketAddr, storage: &Path) {
    let mut stream = connect_with_handshake(addr, handshake("node-a", 0));
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    TcpReplaySession::send_frame(
        &mut stream,
        &TcpReplayFrame::Resume(ReplayResumeRequest {
            peer_id: "node-a".to_owned(),
            cursor: ReplayCursor {
                next_sequence: 0,
                continuity_root: CONTINUITY_ROOT.to_owned(),
            },
            reconstruction_only: true,
        }),
    )
    .unwrap();

    let mut replay = TcpReplayStream::with_root(CONTINUITY_ROOT);
    let mut checkpoint_transferred = false;
    let mut checkpoint_tip = 0;
    loop {
        match TcpReplaySession::read_frame(&mut reader).unwrap() {
            TcpReplayFrame::Checkpoint(checkpoint) => {
                checkpoint.validate().unwrap();
                checkpoint_tip = checkpoint.replay_tip;
                checkpoint_transferred = true;
                fs::write(
                    storage.join("checkpoints").join("transferred.checkpoint"),
                    checkpoint.checkpoint_hash,
                )
                .unwrap();
            }
            TcpReplayFrame::Chunk(chunk) => {
                let record = String::from_utf8(chunk.payload.clone()).unwrap();
                replay.ingest(chunk.clone()).unwrap();
                persist_replay(storage, chunk.sequence, &record);
            }
            TcpReplayFrame::Error(done) if done.code == "recovery_complete" => break,
            other => panic!("unexpected recovery frame: {other:?}"),
        }
    }
    let convergence = checkpoint_transferred && replay.next_sequence == checkpoint_tip;
    fs::write(
        storage.join("recovery-proof.txt"),
        format!(
            "checkpoint_transfer={}\nreplay_synchronization={}\nconvergence={}\ncontinuity_preserved={}\nreplay_tip={}\n",
            checkpoint_transferred,
            replay.next_sequence == checkpoint_tip,
            convergence,
            replay.continuity_root == CONTINUITY_ROOT,
            replay.next_sequence
        ),
    )
    .unwrap();
    assert!(convergence);
}

fn accept_primary_connection(
    stream: TcpStream,
    runtime: &mut NodeRuntime,
    storage: &Path,
    accepted_chunks: &mut Vec<ReplayChunkWireMessage>,
) {
    let mut writer = stream.try_clone().unwrap();
    let mut reader = BufReader::new(stream);
    match TcpReplaySession::read_frame(&mut reader).unwrap() {
        TcpReplayFrame::Handshake(remote) => {
            remote.validate_against(PROTOCOL, CONTINUITY_ROOT).unwrap()
        }
        frame => panic!("handshake expected, got {frame:?}"),
    }
    TcpReplaySession::send_frame(
        &mut writer,
        &TcpReplayFrame::Handshake(handshake("node-b", 0)),
    )
    .unwrap();
    let mut replay = TcpReplayStream::with_root(CONTINUITY_ROOT);
    while let Ok(frame) = TcpReplaySession::read_frame(&mut reader) {
        match frame {
            TcpReplayFrame::Chunk(chunk) => {
                replay.ingest(chunk.clone()).unwrap();
                let record = String::from_utf8(chunk.payload.clone()).unwrap();
                apply_record(runtime, &record);
                persist_replay(storage, chunk.sequence, &record);
                write_tick(storage, runtime.session.tick);
                accepted_chunks.push(chunk.clone());
                let ack = TcpReplaySession::ack("node-b", &chunk);
                if TcpReplaySession::send_frame(&mut writer, &TcpReplayFrame::Ack(ack)).is_err() {
                    break;
                }
            }
            TcpReplayFrame::Checkpoint(checkpoint) => {
                checkpoint.validate().unwrap();
                persist_checkpoint(storage, checkpoint.replay_tip, checkpoint.checkpoint_hash);
            }
            _ => break,
        }
    }
}

fn serve_recovery_connection(
    stream: TcpStream,
    runtime: &NodeRuntime,
    storage: &Path,
    accepted_chunks: &[ReplayChunkWireMessage],
) {
    let mut writer = stream.try_clone().unwrap();
    let mut reader = BufReader::new(stream);
    match TcpReplaySession::read_frame(&mut reader).unwrap() {
        TcpReplayFrame::Handshake(remote) => {
            remote.validate_against(PROTOCOL, CONTINUITY_ROOT).unwrap()
        }
        frame => panic!("handshake expected, got {frame:?}"),
    }
    TcpReplaySession::send_frame(
        &mut writer,
        &TcpReplayFrame::Handshake(handshake("node-b", accepted_chunks.len() as u64)),
    )
    .unwrap();
    match TcpReplaySession::read_frame(&mut reader).unwrap() {
        TcpReplayFrame::Resume(resume) => resume.validate().unwrap(),
        frame => panic!("resume expected, got {frame:?}"),
    }
    let checkpoint = replay_checkpoint(
        accepted_chunks.len() as u64,
        runtime.checkpoint().checkpoint_root,
    );
    TcpReplaySession::send_frame(&mut writer, &TcpReplayFrame::Checkpoint(checkpoint)).unwrap();
    for chunk in accepted_chunks {
        TcpReplaySession::send_frame(&mut writer, &TcpReplayFrame::Chunk(chunk.clone())).unwrap();
    }
    TcpReplaySession::send_frame(
        &mut writer,
        &TcpReplayFrame::Error(ReplayTransportError::new(
            "recovery_complete",
            format!("storage={}", storage.display()),
        )),
    )
    .unwrap();
}

fn connect_with_handshake(addr: SocketAddr, local: ReplayPeerHandshake) -> TcpStream {
    let mut stream = retry_connect(addr);
    TcpReplaySession::send_frame(&mut stream, &TcpReplayFrame::Handshake(local.clone())).unwrap();
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    match TcpReplaySession::read_frame(&mut reader).unwrap() {
        TcpReplayFrame::Handshake(remote) => {
            remote.validate_against(PROTOCOL, CONTINUITY_ROOT).unwrap()
        }
        frame => panic!("handshake expected, got {frame:?}"),
    }
    stream
}

fn retry_connect(addr: SocketAddr) -> TcpStream {
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        match TcpStream::connect(addr) {
            Ok(stream) => return stream,
            Err(err) if Instant::now() < deadline => {
                let _ = err;
                thread::sleep(Duration::from_millis(25));
            }
            Err(err) => panic!("connect failed: {err}"),
        }
    }
}

fn handshake(peer_id: &str, replay_tip: u64) -> ReplayPeerHandshake {
    ReplayPeerHandshake {
        peer_id: peer_id.to_owned(),
        protocol_version: PROTOCOL.to_owned(),
        replay_tip,
        continuity_root: CONTINUITY_ROOT.to_owned(),
        supported_transports: vec!["tcp".to_owned()],
        reconstruction_only: true,
    }
}

fn make_chunk(sequence: u64, record: &str) -> ReplayChunkWireMessage {
    ReplayChunkWireMessage::new(
        "arena-vanguard-real-transport",
        sequence,
        if sequence == 0 {
            "genesis"
        } else {
            CONTINUITY_ROOT
        },
        CONTINUITY_ROOT,
        record.as_bytes().to_vec(),
    )
}

fn send_chunk(stream: &mut TcpStream, storage: &Path, sequence: u64, record: &str) {
    let chunk = make_chunk(sequence, record);
    chunk.validate().unwrap();
    TcpReplaySession::send_frame(stream, &TcpReplayFrame::Chunk(chunk.clone())).unwrap();
    persist_replay(storage, sequence, record);
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    match TcpReplaySession::read_frame(&mut reader).unwrap() {
        TcpReplayFrame::Ack(ack) if ack.accepted && ack.acknowledged_sequence == sequence => {}
        frame => panic!("ack expected, got {frame:?}"),
    }
}

fn send_checkpoint(
    stream: &mut TcpStream,
    storage: &Path,
    replay_tip: u64,
    checkpoint_root: String,
) {
    let checkpoint = replay_checkpoint(replay_tip, checkpoint_root);
    checkpoint.validate().unwrap();
    TcpReplaySession::send_frame(stream, &TcpReplayFrame::Checkpoint(checkpoint.clone())).unwrap();
    persist_checkpoint(storage, replay_tip, checkpoint.checkpoint_hash);
}

fn replay_checkpoint(replay_tip: u64, checkpoint_root: String) -> ReplayCheckpointWireMessage {
    let continuity_root = format!("{CONTINUITY_ROOT}:{checkpoint_root}");
    let checkpoint_hash =
        deterministic_hash(format!("{}:{}", replay_tip, continuity_root).as_bytes());
    ReplayCheckpointWireMessage {
        checkpoint_sequence: replay_tip,
        replay_tip,
        continuity_root,
        checkpoint_hash,
        reconstruction_only: true,
    }
}

fn persist_replay(storage: &Path, sequence: u64, record: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(storage.join("replay.log"))
        .unwrap();
    writeln!(file, "{sequence}:{record}").unwrap();
}

fn persist_checkpoint(storage: &Path, replay_tip: u64, checkpoint_hash: String) {
    fs::write(
        storage
            .join("checkpoints")
            .join(format!("checkpoint-{replay_tip:04}.state")),
        checkpoint_hash,
    )
    .unwrap();
}

fn latest_checkpoint_file(storage: &Path) -> PathBuf {
    let mut entries: Vec<_> = fs::read_dir(storage.join("checkpoints"))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect();
    entries.sort();
    entries.pop().unwrap()
}

fn apply_record(runtime: &mut NodeRuntime, record: &str) {
    let tick = parse_field(record, "tick:");
    let state = parse_field(record, "state:");
    if let Some(tick) = tick {
        runtime.session.tick = tick as u64;
    }
    if let Some(state) = state {
        runtime.session.state = state;
    }
}

fn parse_field(record: &str, prefix: &str) -> Option<i64> {
    record
        .split('|')
        .find_map(|part| part.strip_prefix(prefix))
        .and_then(|value| value.parse::<i64>().ok())
}

fn write_tick(storage: &Path, tick: u64) {
    fs::write(storage.join("progress.tick"), tick.to_string()).unwrap();
}

fn read_tick(path: &Path) -> u64 {
    fs::read_to_string(path).unwrap().trim().parse().unwrap()
}

fn wait_for_tick(path: &Path, target: u64) {
    let deadline = Instant::now() + Duration::from_secs(10);
    while Instant::now() < deadline {
        if path.exists() && read_tick(path) >= target {
            return;
        }
        thread::sleep(Duration::from_millis(25));
    }
    panic!("timed out waiting for {path:?} to reach tick {target}");
}

fn wait_for_file(path: &Path, timeout: Duration) {
    let deadline = Instant::now() + timeout;
    while Instant::now() < deadline {
        if path.exists() {
            return;
        }
        thread::sleep(Duration::from_millis(25));
    }
    panic!("timed out waiting for {path:?}");
}
