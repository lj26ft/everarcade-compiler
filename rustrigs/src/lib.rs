pub mod ai;
pub mod combat;
pub mod crafting;
pub mod dependency;
pub mod deployment;
pub mod dialogue;
pub mod diplomacy;
pub mod economy;
pub mod factions;
pub mod interaction;
pub mod inventory;
pub mod movement;
pub mod package;
pub mod progression;
pub mod quests;
pub mod world;
pub mod xrpl;

pub use contract_api::rustrig::{
    ComposableRustrig, ReplaySafeRustrig, Rustrig, RustrigDescriptor, ValidatedRustrig,
    VersionedRustrig,
};

pub fn library() -> Vec<RustrigDescriptor> {
    let mut rigs = Vec::new();
    rigs.extend(combat::descriptors());
    rigs.extend(inventory::descriptors());
    rigs.extend(quests::descriptors());
    rigs.extend(dialogue::descriptors());
    rigs.extend(economy::descriptors());
    rigs.extend(world::descriptors());
    rigs.extend(xrpl::descriptors());
    rigs.extend(deployment::descriptors());
    rigs
}

pub fn validate_library() -> bool {
    library()
        .iter()
        .all(|rig| rig.deterministic && rig.replay_safe && rig.emits_records_only)
}
