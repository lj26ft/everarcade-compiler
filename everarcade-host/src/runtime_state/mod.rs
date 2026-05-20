pub mod error;
pub mod recovery;
pub mod root;
pub mod serialization;
pub mod snapshot;
pub mod store;

pub use error::RuntimeStateError;
pub use recovery::{load_latest_snapshot, restore_snapshot};
pub use root::compute_state_root;
pub use store::{apply_state_diff, load_state, persist_state, RuntimeStateStore};
