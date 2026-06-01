use serde::{Deserialize, Serialize};

/// Canonical deterministic gameplay primitive contract.
///
/// Rustrigs are pure functions over explicit input. They emit protocol records
/// for the authoritative runtime to apply; they do not mutate authority state or
/// perform IO.
pub trait Rustrig {
    type Input;
    type Output;

    fn execute(input: Self::Input) -> Self::Output;
}

/// A Rustrig whose output can be routed into another deterministic primitive.
pub trait ComposableRustrig: Rustrig {
    type NextInput;

    fn compose(output: Self::Output) -> Self::NextInput;
}

/// Marker and metadata trait for Rustrigs that are safe to replay from records.
pub trait ReplaySafeRustrig: Rustrig {
    const REPLAY_SAFE: bool = true;
    const AUTHORITY_WRITES: bool = false;
    const NETWORK_IO: bool = false;
    const FILESYSTEM_IO: bool = false;
}

/// A Rustrig that can validate explicit input before deterministic execution.
pub trait ValidatedRustrig: Rustrig {
    type ValidationError;

    fn validate(input: &Self::Input) -> Result<(), Self::ValidationError>;
}

/// Version metadata used by Studio, SDK, marketplace, and replay manifests.
pub trait VersionedRustrig: Rustrig {
    const NAME: &'static str;
    const VERSION: &'static str;
    const RECORD_TYPE: &'static str;
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RustrigDescriptor {
    pub name: String,
    pub version: String,
    pub record_type: String,
    pub deterministic: bool,
    pub replay_safe: bool,
    pub emits_records_only: bool,
}

impl RustrigDescriptor {
    pub fn new(
        name: impl Into<String>,
        version: impl Into<String>,
        record_type: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            record_type: record_type.into(),
            deterministic: true,
            replay_safe: true,
            emits_records_only: true,
        }
    }
}

pub mod context;
pub mod output;

pub use context::RustrigContext;
pub use output::RustrigOutput;
