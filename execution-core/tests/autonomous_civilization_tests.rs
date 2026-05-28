use execution_core::{
    autonomous_world_recovery::AutonomousWorldRecoveryState,
    civilization_interaction::CivilizationInteractionState,
    civilization_scheduler::CivilizationSchedulerState, ecology_runtime::EcologyRuntimeState,
    faction_runtime::FactionRuntimeState, governance_runtime::GovernanceRuntimeState,
    procedural_world::ProceduralWorldState, social_memory::SocialMemoryState,
    society_runtime::SocietyRuntimeState,
};

fn evolved<T>(mut state: T, input: &str) -> T
where
    T: Evolvable,
{
    state.evolve(input).unwrap();
    state
}

trait Evolvable {
    fn evolve(&mut self, input: &str) -> Result<(), &'static str>;
}

macro_rules! impl_evolvable {
    ($($t:ty),* $(,)?) => {
        $(impl Evolvable for $t {
            fn evolve(&mut self, input: &str) -> Result<(), &'static str> { self.evolve(input) }
        })*
    };
}

impl_evolvable!(
    FactionRuntimeState,
    SocietyRuntimeState,
    GovernanceRuntimeState,
    EcologyRuntimeState,
    CivilizationInteractionState,
    ProceduralWorldState,
    SocialMemoryState,
    CivilizationSchedulerState,
    AutonomousWorldRecoveryState,
);

#[test]
fn test_faction_governance_equivalence() {
    let a = evolved(FactionRuntimeState::genesis("sol"), "governance-council");
    let b = evolved(FactionRuntimeState::genesis("sol"), "governance-council");
    execution_core::faction_runtime::validation::equivalent(&a, &b).unwrap();
}

#[test]
fn test_society_evolution_equivalence() {
    let a = evolved(SocietyRuntimeState::genesis("sol"), "population-shift");
    let b = evolved(SocietyRuntimeState::genesis("sol"), "population-shift");
    execution_core::society_runtime::validation::equivalent(&a, &b).unwrap();
}

#[test]
fn test_diplomacy_continuity() {
    let a = evolved(GovernanceRuntimeState::genesis("sol"), "treaty-ratified");
    let b = evolved(GovernanceRuntimeState::genesis("sol"), "treaty-ratified");
    execution_core::governance_runtime::validation::equivalent(&a, &b).unwrap();
}

#[test]
fn test_ecology_simulation_equivalence() {
    let a = evolved(EcologyRuntimeState::genesis("sol"), "rainfall-cycle");
    let b = evolved(EcologyRuntimeState::genesis("sol"), "rainfall-cycle");
    execution_core::ecology_runtime::validation::equivalent(&a, &b).unwrap();
}

#[test]
fn test_trade_conflict_equivalence() {
    let a = evolved(
        CivilizationInteractionState::genesis("sol"),
        "trade-before-conflict",
    );
    let b = evolved(
        CivilizationInteractionState::genesis("sol"),
        "trade-before-conflict",
    );
    execution_core::civilization_interaction::validation::equivalent(&a, &b).unwrap();
}

#[test]
fn test_procedural_world_equivalence() {
    let a = evolved(ProceduralWorldState::genesis("sol"), "terrain-seed-7");
    let b = evolved(ProceduralWorldState::genesis("sol"), "terrain-seed-7");
    execution_core::procedural_world::validation::equivalent(&a, &b).unwrap();
}

#[test]
fn test_social_memory_continuity() {
    let mut memory = SocialMemoryState::genesis("sol");
    memory.evolve("relationship-formed").unwrap();
    let restored = execution_core::social_memory::recovery::restore(&memory, &memory).unwrap();
    assert_eq!(memory.append_only_history, restored.append_only_history);
}

#[test]
fn test_civilization_scheduler_equivalence() {
    let a = evolved(
        CivilizationSchedulerState::genesis("sol"),
        "faction-ecology-governance",
    );
    let b = evolved(
        CivilizationSchedulerState::genesis("sol"),
        "faction-ecology-governance",
    );
    execution_core::civilization_scheduler::validation::equivalent(&a, &b).unwrap();
}

#[test]
fn test_civilization_federation_continuity() {
    let local = evolved(
        CivilizationSchedulerState::genesis("federation"),
        "society-ecology-sync",
    );
    let remote = evolved(
        CivilizationSchedulerState::genesis("federation"),
        "society-ecology-sync",
    );
    execution_core::civilization_scheduler::validation::equivalent(&local, &remote).unwrap();
}

#[test]
fn test_autonomous_world_restoration() {
    let world = evolved(
        AutonomousWorldRecoveryState::genesis("sol"),
        "restore-civilization-ecology-memory",
    );
    let restored =
        execution_core::autonomous_world_recovery::recovery::restore(&world, &world).unwrap();
    assert_eq!(world, restored);
}

#[test]
fn test_authority_mutation_rejection() {
    assert!(execution_core::faction_runtime::validation::reject_authority_mutation(true).is_err());
    assert!(execution_core::social_memory::validation::reject_authority_mutation(true).is_err());
    assert!(execution_core::ecology_runtime::validation::reject_authority_mutation(false).is_ok());
}

#[test]
fn test_observer_civilization_hydration() {
    let authoritative = evolved(SocialMemoryState::genesis("observer"), "replay-hydration");
    let reconstructed = evolved(SocialMemoryState::genesis("observer"), "replay-hydration");
    execution_core::social_memory::validation::equivalent(&authoritative, &reconstructed).unwrap();
    assert!(reconstructed.reject_authority_write(true).is_err());
}
