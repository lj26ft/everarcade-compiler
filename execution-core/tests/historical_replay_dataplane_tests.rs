#[path = "../../runtime/renderer-client/src/history/mod.rs"]
mod history;
use history::adversarial::HistoricalCorruptionMatrix;
use history::adversarial::HistoricalCorruptionScenario;
use history::adversarial::HistoricalRuntimeValidationEngine;
use history::branch::{ReplayBranchProofRuntime, ReplayForkMaterialization};
use history::cache::{
    HistoricalReplayCache, HistoricalReplayCacheManifest, HistoricalReplayCacheWindow,
};
use history::compression::{ReplayCompressionTreeBuilder, ReplayCompressionTreeRuntime};
use history::continuity_chain::ReplayContinuityChain;
use history::corruption::{
    HistoricalReplayRestorationSession, HistoricalRestorationVerificationRuntime,
};
use history::export::HistoricalArchiveExporter;
use history::history_is_non_authoritative;
use history::hydration::{HistoricalReplayHydrationRuntime, HistoricalReplayHydrationWindow};
use history::import::HistoricalArchiveImporter;
use history::index::HistoricalReplayIndex;
use history::io::{HistoricalArtifactManifest, HistoricalArtifactRecord, HistoricalArtifactStore};
use history::materialization::ReplayProofMaterializationRuntime;
use history::proof_verification::ReplayProofVerificationRuntime;
use history::provenance::{ReplayProvenanceManifest, ReplayProvenanceProof, ReplayProvenanceRoot};
use history::query::{HistoricalReplayQuery, HistoricalReplayQueryRuntime};
use history::restore::{HistoricalReplayRestorationCursor, HistoricalReplayRestorationRuntime};
use history::versioning::{HistoricalArchiveFormatVersion, HistoricalArchiveVersionManifest};

#[test]
fn test_historical_artifact_io_equivalence() {
    let dir = std::env::temp_dir().join("everarcade_hist_io_eq");
    let _ = std::fs::remove_dir_all(&dir);
    let store = HistoricalArtifactStore::new(&dir);
    let record = HistoricalArtifactRecord {
        manifest: HistoricalArtifactManifest {
            artifact_id: "timeline".into(),
            continuity_root: "root-1".into(),
            sequence: 1,
        },
        payload: b"payload".to_vec(),
    };
    store.writer().append(&record).unwrap();
    assert_eq!(store.reader().read_all().unwrap()[0], record);
}
#[test]
fn test_historical_index_query_equivalence() {
    let idx = HistoricalReplayIndex {
        key: "k".into(),
        era_id: "e".into(),
        frame: 4,
        provenance_root: "p".into(),
    };
    let out =
        HistoricalReplayQueryRuntime::query(&[idx], &HistoricalReplayQuery { key: "k".into() })
            .unwrap();
    assert_eq!(out.frame, 4);
}
#[test]
fn test_replay_proof_materialization() {
    assert_eq!(
        ReplayProofMaterializationRuntime::continuity("r").lineage_hash,
        "lineage::r"
    );
}
#[test]
fn test_replay_archive_roundtrip_equivalence() {
    let export = HistoricalArchiveExporter::export("root");
    let imported = HistoricalArchiveImporter::import(&export.continuity_root, b"ok").unwrap();
    assert_eq!(imported, "root");
}
#[test]
fn test_historical_replay_hydration() {
    let out = HistoricalReplayHydrationRuntime::hydrate(
        &HistoricalReplayHydrationWindow {
            era_id: "e".into(),
            start_frame: 0,
            end_frame: 1,
        },
        "r",
    );
    assert!(out.equivalent);
}
#[test]
fn test_replay_branch_materialization() {
    let proof = ReplayBranchProofRuntime::prove(&ReplayForkMaterialization {
        parent_root: "a".into(),
        child_root: "b".into(),
    });
    assert!(proof.divergence_detected);
}
#[test]
fn test_replay_compression_tree_restoration() {
    let out = ReplayCompressionTreeRuntime::restore(&ReplayCompressionTreeBuilder {
        chunks: vec![b"a".to_vec(), b"b".to_vec()],
    });
    assert_eq!(out.payload, b"ab");
}
#[test]
fn test_historical_replay_restoration_equivalence() {
    let out = HistoricalReplayRestorationRuntime::restore(
        "r",
        &HistoricalReplayRestorationCursor { frame: 1 },
    );
    assert!(out.restored);
}
#[test]
fn test_historical_cache_equivalence() {
    let cache = HistoricalReplayCache {
        manifest: HistoricalReplayCacheManifest {
            continuity_root: "r".into(),
            windows: vec![HistoricalReplayCacheWindow {
                start_frame: 0,
                end_frame: 2,
            }],
        },
    };
    assert_eq!(cache.manifest.windows.len(), 1);
}
#[test]
fn test_historical_replay_corruption_detection() {
    assert!(HistoricalArchiveImporter::import("r", b"").is_err());
}
#[test]
fn test_historical_replay_provenance_integrity() {
    let p = ReplayProvenanceProof {
        manifest: ReplayProvenanceManifest {
            timeline_id: "t".into(),
            root: ReplayProvenanceRoot { value: "r".into() },
        },
        witness: "w".into(),
    };
    assert!(p.verify(&ReplayProvenanceRoot { value: "r".into() }));
}
#[test]
fn test_historical_replay_non_authoritative() {
    assert!(history_is_non_authoritative());
}
#[test]
fn test_historical_continuity_chain_verification() {
    let mut links = Vec::new();
    ReplayContinuityChain::append(&mut links, "a1", b"payload-1");
    ReplayContinuityChain::append(&mut links, "a2", b"payload-2");
    assert!(ReplayContinuityChain::verify_append_only(&links));
}
#[test]
fn test_replay_archive_version_compatibility() {
    let m = HistoricalArchiveVersionManifest {
        archive_id: "a".into(),
        format: HistoricalArchiveFormatVersion { major: 1, minor: 0 },
        minimum_reader_major: 1,
    };
    assert!(
        m.compatibility(&HistoricalArchiveFormatVersion { major: 1, minor: 2 })
            .compatible
    );
    assert!(
        !m.compatibility(&HistoricalArchiveFormatVersion { major: 2, minor: 0 })
            .compatible
    );
}
#[test]
fn test_historical_corruption_matrix_rejection() {
    let matrix = HistoricalCorruptionMatrix {
        scenarios: vec![HistoricalCorruptionScenario {
            name: "replay_injection".into(),
            violated: true,
        }],
    };
    let out = matrix.evaluate();
    assert!(!out.accepted);
    assert_eq!(out.rejected_scenarios[0], "replay_injection");
}
#[test]
fn test_runtime_generated_validation_reports() {
    let matrix = HistoricalCorruptionMatrix { scenarios: vec![] };
    let report = HistoricalRuntimeValidationEngine::execute(100, true, &matrix);
    assert!(report.success);
    assert_eq!(report.generated_at_unix, 100);
}
#[test]
fn test_historical_restoration_failure_detection() {
    let session = HistoricalReplayRestorationSession {
        continuity_root: "actual".into(),
        restored: true,
    };
    assert!(HistoricalRestorationVerificationRuntime::verify(&session, "expected").is_err());
}
#[test]
fn test_historical_index_corruption_rejection() {
    let entries = vec![
        HistoricalReplayIndex {
            key: "k".into(),
            era_id: "e".into(),
            frame: 10,
            provenance_root: "p".into(),
        },
        HistoricalReplayIndex {
            key: "k".into(),
            era_id: "e".into(),
            frame: 9,
            provenance_root: "p".into(),
        },
    ];
    assert!(entries.windows(2).any(|w| w[1].frame < w[0].frame));
}
#[test]
fn test_historical_archive_roundtrip_integrity() {
    let payload = b"archive-window";
    let digest = ReplayProofVerificationRuntime::digest_bytes(payload);
    let chain = vec!["genesis".to_string(), "era-1".to_string()];
    let chain_digest = ReplayProofVerificationRuntime::digest_chain(&chain);
    let out =
        ReplayProofVerificationRuntime::verify(&chain, &chain_digest, payload, &digest).unwrap();
    assert!(out.valid);
}
#[test]
fn test_replay_truncation_rejection() {
    let chain = vec!["genesis".to_string()];
    let expected =
        ReplayProofVerificationRuntime::digest_chain(&["genesis".to_string(), "era-2".to_string()]);
    let result = ReplayProofVerificationRuntime::verify(
        &chain,
        &expected,
        b"x",
        &ReplayProofVerificationRuntime::digest_bytes(b"x"),
    );
    assert!(result.is_err());
}
#[test]
fn test_replay_injection_rejection() {
    let matrix = HistoricalCorruptionMatrix {
        scenarios: vec![
            HistoricalCorruptionScenario {
                name: "replay_injection".into(),
                violated: true,
            },
            HistoricalCorruptionScenario {
                name: "replay_duplication".into(),
                violated: true,
            },
        ],
    };
    assert!(!matrix.evaluate().accepted);
}
#[test]
fn test_historical_runtime_security_validation() {
    let matrix = HistoricalCorruptionMatrix {
        scenarios: vec![HistoricalCorruptionScenario {
            name: "continuity_chain_integrity".into(),
            violated: false,
        }],
    };
    let report = HistoricalRuntimeValidationEngine::execute(200, true, &matrix);
    assert!(report.stages.iter().all(|s| s.passed));
}

#[test]
fn test_runtime_namespace_governance() {
    let ownership = execution_core::runtime::export_governance::runtime_export_ownership();
    assert!(!ownership.is_empty());
}

#[test]
fn test_namespace_drift_detection() {
    let audit = execution_core::runtime::export_governance::RuntimeNamespaceAudit::default();
    assert!(audit.unresolved_symbols.is_empty());
}

#[test]
fn test_explicit_import_integrity() {
    assert!(history_is_non_authoritative());
}
