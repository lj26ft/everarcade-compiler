pub mod capability;
pub mod compatibility;
pub mod descriptor;
pub mod epoch;
pub mod errors;
pub mod exchange;
pub mod governance;
pub mod migration;
pub mod persistence;
pub mod policy;
pub mod quarantine;
pub mod registry;
pub mod replay_continuity;
pub mod routing;
pub mod session;
pub mod state;
pub mod topology;
pub mod upgrade;
pub mod verification;

pub use descriptor::{hash_coordination_descriptor, CoordinationDescriptor};
pub use errors::CoordinationError;
pub use exchange::{verify_coordination_exchange, CoordinationExchange};
pub use policy::{verify_coordination_policy, CoordinationPolicy};
pub use quarantine::{verify_coordination_quarantine, CoordinationQuarantine};
pub use registry::{
    hash_coordination_registry, update_coordination_registry, CoordinationRegistry,
};
pub use session::{hash_coordination_session, verify_coordination_session, CoordinationSession};
pub use state::{hash_coordination_state, verify_coordination_state, CoordinationState};
pub use verification::{verify_coordination, CoordinationVerificationReport};
