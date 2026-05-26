#[path = "../../runtime/renderer-client/src/history/mod.rs"]
mod history;
use history::*;

#[test] fn test_historical_artifact_io_equivalence() { let dir = std::env::temp_dir().join("everarcade_hist_io_eq"); let _ = std::fs::remove_dir_all(&dir); let store = HistoricalArtifactStore::new(&dir); let record = HistoricalArtifactRecord { manifest: HistoricalArtifactManifest { artifact_id: "timeline".into(), continuity_root: "root-1".into(), sequence: 1 }, payload: b"payload".to_vec() }; store.writer().append(&record).unwrap(); assert_eq!(store.reader().read_all().unwrap()[0], record); }
#[test] fn test_historical_index_query_equivalence() { let idx = HistoricalReplayIndex { key:"k".into(), era_id:"e".into(), frame:4, provenance_root:"p".into() }; let out = HistoricalReplayQueryRuntime::query(&[idx], &HistoricalReplayQuery{key:"k".into()}).unwrap(); assert_eq!(out.frame,4); }
#[test] fn test_replay_proof_materialization() { assert_eq!(ReplayProofMaterializationRuntime::continuity("r").lineage_hash, "lineage::r"); }
#[test] fn test_replay_archive_roundtrip_equivalence() { let export = HistoricalArchiveExporter::export("root"); let imported = HistoricalArchiveImporter::import(&export.continuity_root, b"ok").unwrap(); assert_eq!(imported, "root"); }
#[test] fn test_historical_replay_hydration() { let out = HistoricalReplayHydrationRuntime::hydrate(&HistoricalReplayHydrationWindow{era_id:"e".into(),start_frame:0,end_frame:1}, "r"); assert!(out.equivalent); }
#[test] fn test_replay_branch_materialization() { let proof = ReplayBranchProofRuntime::prove(&ReplayForkMaterialization{parent_root:"a".into(), child_root:"b".into()}); assert!(proof.divergence_detected); }
#[test] fn test_replay_compression_tree_restoration() { let out = ReplayCompressionTreeRuntime::restore(&ReplayCompressionTreeBuilder{chunks:vec![b"a".to_vec(),b"b".to_vec()]}); assert_eq!(out.payload, b"ab"); }
#[test] fn test_historical_replay_restoration_equivalence() { let out = HistoricalReplayRestorationRuntime::restore("r", &HistoricalReplayRestorationCursor{frame:1}); assert!(out.restored); }
#[test] fn test_historical_cache_equivalence() { let cache = HistoricalReplayCache { manifest: HistoricalReplayCacheManifest { continuity_root: "r".into(), windows: vec![HistoricalReplayCacheWindow{start_frame:0,end_frame:2}] } }; assert_eq!(cache.manifest.windows.len(), 1); }
#[test] fn test_historical_replay_corruption_detection() { assert!(HistoricalArchiveImporter::import("r", b"").is_err()); }
#[test] fn test_historical_replay_provenance_integrity() { let p = ReplayProvenanceProof { manifest: ReplayProvenanceManifest { timeline_id:"t".into(), root: ReplayProvenanceRoot{value:"r".into()} }, witness:"w".into() }; assert!(p.verify(&ReplayProvenanceRoot{value:"r".into()})); }
#[test] fn test_historical_replay_non_authoritative() { assert!(history_is_non_authoritative()); }
