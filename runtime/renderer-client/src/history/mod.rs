pub mod adversarial;
pub mod anchor;
pub mod archive;
pub mod branch;
pub mod compression;
pub mod continuity;
pub mod era;
pub mod federation;
pub mod hydration;
pub mod index;
pub mod provenance;
pub mod query;
pub mod timeline;
pub mod verification;

pub use adversarial::*;
pub use anchor::*;
pub use archive::*;
pub use branch::*;
pub use compression::*;
pub use continuity::*;
pub use era::*;
pub use federation::*;
pub use hydration::*;
pub use index::*;
pub use provenance::*;
pub use query::*;
pub use timeline::*;
pub use verification::*;

pub fn history_is_non_authoritative() -> bool { true }
