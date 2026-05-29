use crate::{diagnostic, hot_reload, reject_authority_bypass, CreatorDiagnostic};

pub fn validate_hot_reload(checkpoint_root: &str) -> CreatorDiagnostic { diagnostic("hot-reload-validation", &["checkpoint", checkpoint_root, "replay-continuity"] ) }

pub fn hot_reload_restoration(checkpoint_root: &str) -> bool { hot_reload::recovery::restore_after_reload(checkpoint_root) == hot_reload::recovery::restore_after_reload(checkpoint_root) }

pub fn reject_invalid_reload(authority_mutation: bool) -> Result<(), &'static str> { reject_authority_bypass(authority_mutation) }
