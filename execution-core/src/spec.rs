// FILE: execution-core/src/spec.rs
//
// EVERARCADE EXECUTION SPEC v1
//
// THESE RULES ARE NOW IMMUTABLE FOR v1

/// Execution MUST be deterministic
pub const RULE_1: &str = "deterministic_only";

/// Execution MUST be DAG-ordered
pub const RULE_2: &str = "canonical_dag_ordering";

/// Execution MUST use BTreeMap/BTreeSet ONLY
pub const RULE_3: &str = "canonical_collections_only";

/// Execution MUST NOT use IO/clock/network
pub const RULE_4: &str = "pure_vm";

/// State transitions MUST be replayable
pub const RULE_5: &str = "replay_safe";

/// Receipts MUST be cryptographically chained
pub const RULE_6: &str = "hash_chained_execution";
