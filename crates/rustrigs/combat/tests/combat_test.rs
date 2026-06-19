use everarcade_rustrig_combat::{
    attack, replay_with_config as replay, CombatConfig, CombatEntity, CombatError, CombatInput,
    CombatState, RUSTRIG_ID,
};

fn config() -> CombatConfig {
    CombatConfig::new(50, ["slash", "pierce"])
}

fn state() -> CombatState {
    CombatState::new()
        .with_entity(CombatEntity::new("attacker", 100, 100))
        .with_entity(CombatEntity::new("target", 40, 40))
}

fn input(damage: u64) -> CombatInput {
    CombatInput {
        attacker_id: "attacker".to_owned(),
        target_id: "target".to_owned(),
        damage,
        attack_type: "slash".to_owned(),
        tick: 7,
    }
}

#[test]
fn valid_attack_updates_health_and_receipt() {
    let output = attack(&state(), &config(), input(15)).unwrap();
    let target = output.state.entity("target").unwrap();

    assert_eq!(target.current_health, 25);
    assert!(target.alive);
    assert_eq!(output.receipt.rustrig_id, RUSTRIG_ID);
    assert_eq!(output.receipt.damage, 15);
    assert_eq!(output.receipt.health_before, 40);
    assert_eq!(output.receipt.health_after, 25);
    assert_eq!(output.receipt.post_state_root, output.state_root);
}

#[test]
fn target_death_sets_alive_false() {
    let output = attack(&state(), &config(), input(40)).unwrap();
    let target = output.state.entity("target").unwrap();

    assert_eq!(target.current_health, 0);
    assert!(!target.alive);
    assert_eq!(output.receipt.health_after, 0);
}

#[test]
fn overkill_attack_floors_health_at_zero() {
    let output = attack(&state(), &config(), input(50)).unwrap();
    let target = output.state.entity("target").unwrap();

    assert_eq!(target.current_health, 0);
    assert_eq!(output.receipt.health_before, 40);
    assert_eq!(output.receipt.health_after, 0);
}

#[test]
fn rejects_invalid_attacker() {
    let err = attack(
        &state(),
        &config(),
        CombatInput {
            attacker_id: "missing".to_owned(),
            ..input(10)
        },
    );
    assert_eq!(err, Err(CombatError::AttackerMissing));
}

#[test]
fn rejects_invalid_target() {
    let err = attack(
        &state(),
        &config(),
        CombatInput {
            target_id: "missing".to_owned(),
            ..input(10)
        },
    );
    assert_eq!(err, Err(CombatError::TargetMissing));
}

#[test]
fn rejects_dead_attacker() {
    let dead_attacker_state = CombatState::new()
        .with_entity(CombatEntity::new("attacker", 0, 100))
        .with_entity(CombatEntity::new("target", 40, 40));

    assert_eq!(
        attack(&dead_attacker_state, &config(), input(10)),
        Err(CombatError::AttackerDead)
    );
}

#[test]
fn rejects_dead_target() {
    let dead_target_state = CombatState::new()
        .with_entity(CombatEntity::new("attacker", 100, 100))
        .with_entity(CombatEntity::new("target", 0, 40));

    assert_eq!(
        attack(&dead_target_state, &config(), input(10)),
        Err(CombatError::TargetDead)
    );
}

#[test]
fn rejects_zero_damage() {
    assert_eq!(
        attack(&state(), &config(), input(0)),
        Err(CombatError::DamageMustBePositive)
    );
}

#[test]
fn rejects_damage_above_configured_max_damage() {
    assert_eq!(
        attack(&state(), &config(), input(51)),
        Err(CombatError::DamageExceedsMax)
    );
}

#[test]
fn rejects_undeclared_attack_type() {
    assert_eq!(
        attack(
            &state(),
            &config(),
            CombatInput {
                attack_type: "fireball".to_owned(),
                ..input(10)
            },
        ),
        Err(CombatError::AttackTypeNotDeclared)
    );
}

#[test]
fn replay_equivalence() {
    let inputs = [
        input(10),
        CombatInput {
            tick: 8,
            damage: 5,
            ..input(5)
        },
    ];
    let manual = attack(
        &attack(&state(), &config(), inputs[0].clone())
            .unwrap()
            .state,
        &config(),
        inputs[1].clone(),
    )
    .unwrap()
    .state;
    let replayed = replay(&state(), &config(), &inputs).unwrap();

    assert_eq!(manual, replayed);
}

#[test]
fn root_equivalence() {
    let first = attack(&state(), &config(), input(15)).unwrap();
    let second = attack(&state(), &config(), input(15)).unwrap();

    assert_eq!(first, second);
    assert_eq!(first.state_root, second.state_root);
}
