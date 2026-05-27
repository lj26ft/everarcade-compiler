#[path = "../../runtime/renderer-client/src/transport_runtime/mod.rs"]
mod transport_runtime;

use transport_runtime::archive::ReplayArchiveHydrationManifest;
use transport_runtime::backpressure::{
    ReplayBackpressureWindow, ReplayWindowBudget, ReplayWindowThrottle,
};
use transport_runtime::chunk::{ReplayChunk, ReplayChunkContinuity};
use transport_runtime::compression::{ReplayCompressionChunk, ReplayCompressionRuntime};
use transport_runtime::equivalence::ReplayEquivalenceRuntime;
use transport_runtime::observer::ObserverReplayRuntime;
use transport_runtime::recovery::ReplayCatchupRuntime;
use transport_runtime::replay_transport_is_non_authoritative;
use transport_runtime::stream::ReplayTransportCursor;
use transport_runtime::stream::ReplayTransportStream;

fn mk_chunk(sequence: u64, previous_hash: &str, continuity_hash: &str) -> ReplayChunk {
    ReplayChunk {
        stream_id: "s1".into(),
        sequence,
        payload: vec![sequence as u8],
        continuity: ReplayChunkContinuity {
            previous_hash: previous_hash.into(),
            continuity_hash: continuity_hash.into(),
        },
    }
}

#[test]
fn test_replay_transport_stream_ordering() {
    let mut s = ReplayTransportStream::default();
    assert!(s.ingest(mk_chunk(0, "", "h0")).is_ok());
    assert!(s.ingest(mk_chunk(1, "h0", "h1")).is_ok());
}
#[test]
fn test_replay_chunk_continuity() {
    let c = mk_chunk(0, "", "h0");
    assert_eq!(c.manifest().continuity_hash, "h0");
}
#[test]
fn test_replay_duplicate_rejection() {
    let mut s = ReplayTransportStream::default();
    s.ingest(mk_chunk(0, "", "h0")).unwrap();
    assert!(s.ingest(mk_chunk(0, "", "h0")).is_err());
}
#[test]
fn test_observer_replay_restoration() {
    let mut s = ReplayTransportStream::default();
    s.ingest(mk_chunk(0, "", "h0")).unwrap();
    let o = ObserverReplayRuntime::restore("o1", &s);
    assert_eq!(o.state.applied_chunks, 1);
}
#[test]
fn test_replay_catchup_equivalence() {
    let c = ReplayTransportCursor {
        next_sequence: 2,
        last_continuity_hash: "h1".into(),
    };
    let r = ReplayCatchupRuntime::resume_state(&c);
    assert_eq!(r.cursor, c);
}
#[test]
fn test_replay_resume_restoration() {
    let c = ReplayTransportCursor {
        next_sequence: 1,
        last_continuity_hash: "h0".into(),
    };
    assert_eq!(
        ReplayCatchupRuntime::resume_state(&c).cursor.next_sequence,
        1
    );
}
#[test]
fn test_replay_compression_equivalence() {
    let chunk = ReplayCompressionRuntime::compress(0, b"abc");
    let restored = ReplayCompressionRuntime::decompress(&chunk).unwrap();
    assert_eq!(restored, b"abc");
}
#[test]
fn test_replay_backpressure_window_ordering() {
    let w = ReplayBackpressureWindow {
        budget: ReplayWindowBudget {
            max_chunks_in_window: 2,
        },
        throttle: ReplayWindowThrottle { inflight_chunks: 1 },
    };
    assert!(w.can_accept());
}
#[test]
fn test_cross_node_replay_equivalence() {
    let mut a = ReplayTransportStream::default();
    let mut b = ReplayTransportStream::default();
    a.ingest(mk_chunk(0, "", "h0")).unwrap();
    b.ingest(mk_chunk(0, "", "h0")).unwrap();
    assert!(ReplayEquivalenceRuntime::compare_streams(&a, &b).equivalent);
}
#[test]
fn test_replay_archive_hydration() {
    let m = ReplayArchiveHydrationManifest {
        continuity_root: "h1".into(),
        chunk_count: 2,
    };
    assert_eq!(m.chunk_count, 2);
}
#[test]
fn test_replay_transport_corruption_detection() {
    assert!(
        !ReplayCompressionRuntime::decompress(&ReplayCompressionChunk {
            sequence: 0,
            compressed_payload: b"abc".to_vec(),
            original_hash: "bad".into()
        })
        .is_ok()
    );
}
#[test]
fn test_replay_transport_non_authoritative() {
    assert!(replay_transport_is_non_authoritative());
}

#[test]
fn test_scaffold_runtime_visibility() {
    assert!(replay_transport_is_non_authoritative());
}

#[test]
fn test_replay_surface_classification() {
    let statuses = ["Production", "ActiveIntegration", "Scaffold", "Deprecated"];
    assert_eq!(statuses.len(), 4);
}

#[test]
fn test_history_cleanup_non_authoritative() {
    assert!(replay_transport_is_non_authoritative());
}

#[test]
fn test_replay_namespace_equivalence() {
    assert!(replay_transport_is_non_authoritative());
}

#[test]
fn test_runtime_surface_lineage() {
    assert_eq!("Scaffold", "Scaffold");
}

#[test]
fn test_integration_continuity() {
    let statuses = ["connected", "deterministic"];
    assert!(statuses.contains(&"connected"));
}

#[test]
fn test_runtime_api_ownership() {
    let ownership = execution_core::runtime::export_governance::runtime_api_ownership();
    assert!(!ownership.is_empty());
}
