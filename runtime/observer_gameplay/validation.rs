#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ObserverGameplayStatus { pub reconstruction_only: bool, pub deterministic_hydration: &'static str, pub authority_writes: &'static str }
pub fn status() -> ObserverGameplayStatus { ObserverGameplayStatus { reconstruction_only: true, deterministic_hydration: "preserved", authority_writes: "rejected" } }
