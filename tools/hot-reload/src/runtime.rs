use crate::stable_hash;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuntimeReloadPlan { pub checkpoint_root: String, pub runtime_root_after_reload: String }

pub fn plan_runtime_reload(checkpoint_root: &str, asset_root: &str) -> RuntimeReloadPlan { RuntimeReloadPlan { checkpoint_root: checkpoint_root.to_owned(), runtime_root_after_reload: stable_hash(&["reload", checkpoint_root, asset_root]) } }
