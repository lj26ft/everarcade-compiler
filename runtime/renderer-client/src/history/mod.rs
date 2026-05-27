#![allow(dead_code)]
pub mod adversarial;
pub mod anchor;
pub mod archive;
pub mod branch;
pub mod cache;
pub mod compression;
pub mod continuity;
pub mod continuity_chain;
pub mod corruption;
pub mod era;
pub mod export;
pub mod federation;
pub mod hydration;
pub mod import;
pub mod index;
pub mod io;
pub mod materialization;
pub mod proof_verification;
pub mod provenance;
pub mod query;
pub mod restore;
pub mod runtime_validation;
pub mod storage;
pub mod timeline;
pub mod verification;
pub mod versioning;

pub use adversarial::*;
pub use anchor::*;
pub use archive::*;
pub use branch::*;
pub use cache::*;
pub use compression::*;
pub use continuity::*;
pub use continuity_chain::*;
pub use corruption::*;
pub use era::*;
pub use export::*;
pub use federation::*;
pub use hydration::*;
pub use import::*;
pub use index::*;
pub use io::*;
pub use materialization::*;
pub use proof_verification::*;
pub use provenance::*;
pub use query::*;
pub use restore::*;
pub use runtime_validation::*;
pub use storage::*;
pub use timeline::*;
pub use verification::*;
pub use versioning::*;

pub fn history_is_non_authoritative() -> bool {
    true
}
