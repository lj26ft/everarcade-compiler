use serde::{Deserialize, Serialize};

/// Canonical deterministic input context for executable Rustrigs.
///
/// This context intentionally contains no wall-clock time, host randomness,
/// filesystem handles, network handles, or authority state references. Runtime
/// callers provide every value explicitly so replay can reconstruct the same
/// input byte-for-byte.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RustrigContext {
    pub world_root: String,
    pub replay_root: String,
    pub checkpoint_root: String,
    pub actor_id: String,
    pub tick: u64,
    pub input_hash: String,
    pub protocol_version: String,
}
