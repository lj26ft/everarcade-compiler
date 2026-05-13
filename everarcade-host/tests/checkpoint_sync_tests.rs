use everarcade_host::checkpoint_sync::checkpoint_validation::lineage_is_continuous;
#[test] fn checkpoint_lineage_continuity(){assert!(lineage_is_continuous([1;32],[1;32]));assert!(!lineage_is_continuous([1;32],[2;32]));}
