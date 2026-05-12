#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LifecycleState { Created, Activated, Suspended, Migrated, Archived, Restored }
