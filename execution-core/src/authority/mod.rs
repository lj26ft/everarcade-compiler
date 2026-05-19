pub mod descriptor;
pub mod epoch;
pub mod errors;
pub mod handoff;
pub mod policy;
pub mod proof;
pub mod registry;
pub mod rotation;
pub mod verification;

pub use descriptor::{hash_authority_descriptor, AuthorityDescriptor};
pub use epoch::{hash_authority_epoch, verify_epoch_transition, AuthorityEpoch};
pub use errors::AuthorityError;
pub use handoff::{hash_authority_handoff, verify_handoff, AuthorityHandoff};
pub use policy::{verify_execution_policy, ExecutionPolicy};
pub use proof::{verify_execution_authority, ExecutionAuthorityProof};
pub use registry::{update_authority_registry, AuthorityRegistry};
pub use rotation::{verify_rotation_policy, RotationPolicy};
pub use verification::{verify_authority_chain, AuthorityVerificationReport};
