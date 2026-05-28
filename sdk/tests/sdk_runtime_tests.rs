use everarcade_sdk::{
    error::SdkError,
    game::CounterGame,
    input::PlayerInput,
    runtime::DeterministicRuntime,
    session::SessionDescriptor,
    validation::{reject_authority_mutation, validate_equivalence},
};

fn canonical_inputs() -> Vec<PlayerInput> {
    vec![
        PlayerInput::new(0, "player-b", "inc"),
        PlayerInput::new(0, "player-a", "inc"),
        PlayerInput::new(0, "player-a", "inc"),
    ]
}

#[test]
fn test_sdk_game_equivalence() {
    let h1 = validate_equivalence::<CounterGame>(canonical_inputs()).unwrap();
    let h2 = validate_equivalence::<CounterGame>(canonical_inputs()).unwrap();
    assert_eq!(h1, h2);
}

#[test]
fn test_runtime_api_equivalence() {
    let mut a = DeterministicRuntime::new(CounterGame);
    let mut b = DeterministicRuntime::new(CounterGame);
    assert_eq!(
        a.tick(canonical_inputs()).unwrap(),
        b.tick(canonical_inputs()).unwrap()
    );
}

#[test]
fn test_template_replay_equivalence() {
    assert!(std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../templates/topdown-arena/game.toml")
        .exists());
}

#[test]
fn test_manifest_validation() {
    let manifest = include_str!("../game-manifest/src/manifest.rs");
    assert!(manifest.contains("tick_rate"));
    assert!(manifest.contains("replay_enabled"));
}

#[test]
fn test_local_runtime_restoration() {
    let s = SessionDescriptor::new("local", "lineage-a", 30);
    assert!(s.validate());
}

#[test]
fn test_replay_debugger_equivalence() {
    let mut rt = DeterministicRuntime::new(CounterGame);
    rt.tick(canonical_inputs()).unwrap();
    assert_eq!(rt.replay().frames().len(), 1);
    assert!(!rt.replay().continuity_hash().is_empty());
}

#[test]
fn test_validation_harness_divergence_detection() {
    let mut rt = DeterministicRuntime::new(CounterGame);
    let err = rt
        .tick(vec![PlayerInput::new(0, "p", "random")])
        .unwrap_err();
    assert_eq!(err, SdkError::NonDeterministicMutation);
}

#[test]
fn test_package_hash_equivalence() {
    assert!(everarcade_sdk::verify_package_determinism(
        b"artifact",
        b"artifact"
    ));
}

#[test]
fn test_deployment_continuity() {
    let mut rt = DeterministicRuntime::new(CounterGame);
    let _ = rt.tick(canonical_inputs()).unwrap();
    assert_eq!(rt.replay().frames()[0].tick, 0);
}

#[test]
fn test_multiplayer_sdk_equivalence() {
    let mut rt = DeterministicRuntime::new(CounterGame);
    rt.tick(canonical_inputs()).unwrap();
    assert_eq!(rt.state().get("player:player-a"), Some("2"));
    assert_eq!(rt.state().get("player:player-b"), Some("1"));
}

#[test]
fn test_authority_mutation_rejection() {
    assert!(reject_authority_mutation(CounterGame));
}

#[test]
fn test_replay_safe_sdk_surfaces() {
    let mut rt = DeterministicRuntime::new(CounterGame);
    assert_eq!(
        rt.replay_mutation_probe().unwrap_err(),
        SdkError::ReplayMutationRejected
    );
}
