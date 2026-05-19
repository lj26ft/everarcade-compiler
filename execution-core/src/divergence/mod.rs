pub mod detection;
pub mod errors;
pub mod fork;
pub mod policy;
pub mod proof;
pub mod reconciliation;
pub mod registry;
pub mod verification;
pub mod window;

pub use detection::{detect_divergence, DivergenceDetectionReport};
pub use fork::{hash_continuity_fork, ContinuityFork};
pub use policy::{verify_divergence_policy, DivergencePolicy};
pub use proof::{verify_divergence_proof, DivergenceProof};
pub use reconciliation::{verify_reconciliation_boundary, ReconciliationBoundary};
pub use registry::{register_divergence, DivergenceRegistry};
pub use verification::{verify_divergence, DivergenceVerificationReport};
pub use window::{verify_divergence_window, DivergenceWindow};
