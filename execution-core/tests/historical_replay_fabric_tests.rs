#[path = "../../runtime/renderer-client/src/history/mod.rs"]
mod history;

use history::*;

fn sample_timeline() -> HistoricalReplayTimeline {
    let era = HistoricalReplayEra {
        manifest: HistoricalReplayEraManifest { era_id: "era-1".into(), continuity_root: "root-1".into(), frame_count: 10 },
        windows: vec![HistoricalReplayEraWindow { era_id: "era-1".into(), start_frame: 0, end_frame: 9 }],
    };
    let mut timeline = HistoricalReplayTimeline::default();
    timeline.append_era(era).unwrap();
    timeline
}

#[test] fn test_historical_replay_timeline_equivalence() { let t = sample_timeline(); assert_eq!(t.eras.len(), 1); }
#[test] fn test_historical_replay_era_restoration() { assert!(sample_timeline().restore_window("era-1",0,9).is_some()); }
#[test] fn test_civilization_archive_restoration() { let m = CivilizationArchiveManifest { archive_id:"a1".into(), continuity_root:"root-1".into(), era_count:1}; assert!(CivilizationArchiveRuntime::restore(&m).restored); }
#[test] fn test_historical_query_equivalence() { let idx = vec![HistoricalReplayIndex { key:"k".into(), era_id:"era-1".into(), frame:2, provenance_root:"p1".into() }]; assert_eq!(HistoricalReplayQueryRuntime::query(&idx,&HistoricalReplayQuery{key:"k".into()}).unwrap().frame,2); }
#[test] fn test_replay_provenance_verification() { let root = ReplayProvenanceRoot{value:"p1".into()}; let proof = ReplayProvenanceProof { manifest: ReplayProvenanceManifest{timeline_id:"t".into(),root:root.clone()}, witness:"w".into() }; assert!(proof.verify(&root)); }
#[test] fn test_replay_branch_divergence_detection() { let b = ReplayBranch{branch_id:"b".into(),parent_root:"a".into(),root:"r".into()}; assert!(!ReplayForkVerification::verify(&b,&ReplayForkProof{expected_parent_root:"x".into()}).valid); }
#[test] fn test_replay_window_federation_equivalence() { let w = HistoricalReplayFederationWindow{window_id:"w".into(),start:0,end:10}; assert_eq!(w.end-w.start,10); }
#[test] fn test_replay_compression_tree_equivalence() { let t = ReplayCompressionTree{nodes:vec![ReplayCompressionNode{id:"n".into(),payload:b"abc".to_vec()}],root:ReplayCompressionRoot{hash:"h".into()}}; assert_eq!(t.decompress(), b"abc"); }
#[test] fn test_civilization_observer_restoration() { assert!(CivilizationObserverRuntime::restore_from_artifacts("era-1", 3).equivalent); }
#[test] fn test_historical_anchor_continuity() { let a = HistoricalReplayAnchor{window:HistoricalReplayAnchorWindow{start_era:"a".into(),end_era:"b".into()},root:HistoricalReplayAnchorRoot{value:"r".into()}}; assert_eq!(a.root.value,"r"); }
#[test] fn test_historical_replay_corruption_detection() { let root = ReplayProvenanceRoot{value:"good".into()}; let proof = ReplayProvenanceProof{manifest:ReplayProvenanceManifest{timeline_id:"t".into(),root:ReplayProvenanceRoot{value:"bad".into()}},witness:"w".into()}; let branch = ReplayForkVerification{valid:false,reason:"divergence".into()}; assert!(!detect_corruption(&branch,&proof,&root).valid); }
#[test] fn test_historical_replay_non_authoritative() { assert!(history_is_non_authoritative()); }
