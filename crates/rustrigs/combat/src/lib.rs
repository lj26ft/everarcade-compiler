use std::collections::{BTreeMap, BTreeSet};

pub const RUSTRIG_ID: &str = "combat.attack";
pub const RUSTRIG_VERSION: &str = "1.0.0";
pub const CERTIFICATION_STATUS: &str = "RUSTRIG COMBAT CERTIFICATION: PASS";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CombatConfig {
    pub max_damage: u64,
    pub declared_attack_types: BTreeSet<String>,
}

impl CombatConfig {
    pub fn new(max_damage: u64, attack_types: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            max_damage,
            declared_attack_types: attack_types.into_iter().map(Into::into).collect(),
        }
    }

    pub fn declares_attack_type(&self, attack_type: &str) -> bool {
        self.declared_attack_types.contains(attack_type)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CombatEntity {
    pub entity_id: String,
    pub current_health: u64,
    pub max_health: u64,
    pub alive: bool,
}

impl CombatEntity {
    pub fn new(entity_id: impl Into<String>, current_health: u64, max_health: u64) -> Self {
        let current_health = current_health.min(max_health);
        Self {
            entity_id: entity_id.into(),
            current_health,
            max_health,
            alive: current_health > 0,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CombatInput {
    pub attacker_id: String,
    pub target_id: String,
    pub damage: u64,
    pub attack_type: String,
    pub tick: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CombatReceipt {
    pub receipt_id: String,
    pub rustrig_id: String,
    pub version: String,
    pub attacker_id: String,
    pub target_id: String,
    pub damage: u64,
    pub attack_type: String,
    pub health_before: u64,
    pub health_after: u64,
    pub tick: u64,
    pub input_hash: String,
    pub pre_state_root: String,
    pub post_state_root: String,
    pub authority: String,
    pub result: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CombatLogEntry {
    pub receipt_id: String,
    pub attacker_id: String,
    pub target_id: String,
    pub damage: u64,
    pub attack_type: String,
    pub health_before: u64,
    pub health_after: u64,
    pub tick: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CombatState {
    pub entities: BTreeMap<String, CombatEntity>,
    pub health: BTreeMap<String, u64>,
    pub combat_log: Vec<CombatLogEntry>,
    pub receipts: BTreeMap<String, CombatReceipt>,
}

impl CombatState {
    pub fn new() -> Self {
        Self {
            entities: BTreeMap::new(),
            health: BTreeMap::new(),
            combat_log: Vec::new(),
            receipts: BTreeMap::new(),
        }
    }

    pub fn with_entity(mut self, entity: CombatEntity) -> Self {
        self.health
            .insert(entity.entity_id.clone(), entity.current_health);
        self.entities.insert(entity.entity_id.clone(), entity);
        self
    }

    pub fn entity(&self, entity_id: &str) -> Option<&CombatEntity> {
        self.entities.get(entity_id)
    }

    pub fn state_root(&self) -> String {
        digest(&self.canonical())
    }

    fn canonical(&self) -> String {
        let entities = self
            .entities
            .iter()
            .map(|(id, entity)| {
                format!(
                    "entity:{id}:health={}:max={}:alive={}",
                    entity.current_health, entity.max_health, entity.alive
                )
            })
            .collect::<Vec<_>>()
            .join("|");
        let health = self
            .health
            .iter()
            .map(|(id, value)| format!("health:{id}:{value}"))
            .collect::<Vec<_>>()
            .join("|");
        let log = self
            .combat_log
            .iter()
            .map(|entry| {
                format!(
                    "log:{}:{}:{}:{}:{}:{}:{}:{}",
                    entry.receipt_id,
                    entry.attacker_id,
                    entry.target_id,
                    entry.damage,
                    entry.attack_type,
                    entry.health_before,
                    entry.health_after,
                    entry.tick
                )
            })
            .collect::<Vec<_>>()
            .join("|");
        let receipts = self
            .receipts
            .iter()
            .map(|(id, receipt)| {
                format!(
                    "receipt:{id}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}",
                    receipt.rustrig_id,
                    receipt.version,
                    receipt.attacker_id,
                    receipt.target_id,
                    receipt.damage,
                    receipt.attack_type,
                    receipt.health_before,
                    receipt.health_after,
                    receipt.tick,
                    receipt.input_hash,
                    receipt.pre_state_root,
                    receipt.result
                )
            })
            .collect::<Vec<_>>()
            .join("|");
        format!("entities=[{entities}];health=[{health}];combat_log=[{log}];receipts=[{receipts}]")
    }
}

impl Default for CombatState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CombatOutput {
    pub receipt: CombatReceipt,
    pub state: CombatState,
    pub state_root: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CombatError {
    AttackerMissing,
    TargetMissing,
    AttackerDead,
    TargetDead,
    DamageMustBePositive,
    DamageExceedsMax,
    AttackTypeNotDeclared,
}

pub fn attack(
    state: &CombatState,
    config: &CombatConfig,
    input: CombatInput,
) -> Result<CombatOutput, CombatError> {
    validate(state, config, &input)?;

    let pre_state_root = state.state_root();
    let input_hash = digest(&canonical_input(&input));
    let mut next_state = state.clone();
    let target = next_state
        .entities
        .get_mut(&input.target_id)
        .expect("target existence validated before mutation");
    let health_before = target.current_health;
    let health_after = health_before.saturating_sub(input.damage);
    target.current_health = health_after;
    target.alive = health_after > 0;
    next_state
        .health
        .insert(input.target_id.clone(), health_after);

    let post_without_receipt_root = next_state.state_root();
    let receipt_id = digest(&format!(
        "{RUSTRIG_ID}|{RUSTRIG_VERSION}|{input_hash}|{pre_state_root}|{post_without_receipt_root}|{}|{}|{}|{}|{}",
        input.attacker_id, input.target_id, input.damage, health_before, health_after
    ));
    let receipt = CombatReceipt {
        receipt_id: receipt_id.clone(),
        rustrig_id: RUSTRIG_ID.to_owned(),
        version: RUSTRIG_VERSION.to_owned(),
        attacker_id: input.attacker_id.clone(),
        target_id: input.target_id.clone(),
        damage: input.damage,
        attack_type: input.attack_type.clone(),
        health_before,
        health_after,
        tick: input.tick,
        input_hash,
        pre_state_root,
        post_state_root: String::new(),
        authority: "world".to_owned(),
        result: "accepted".to_owned(),
    };
    next_state.combat_log.push(CombatLogEntry {
        receipt_id,
        attacker_id: input.attacker_id,
        target_id: input.target_id,
        damage: input.damage,
        attack_type: input.attack_type,
        health_before,
        health_after,
        tick: input.tick,
    });
    next_state
        .receipts
        .insert(receipt.receipt_id.clone(), receipt.clone());
    let post_state_root = next_state.state_root();
    let mut receipt = receipt;
    receipt.post_state_root = post_state_root.clone();
    next_state
        .receipts
        .insert(receipt.receipt_id.clone(), receipt.clone());

    Ok(CombatOutput {
        receipt,
        state: next_state,
        state_root: post_state_root,
    })
}

pub fn replay_with_config(
    initial_state: &CombatState,
    config: &CombatConfig,
    inputs: &[CombatInput],
) -> Result<CombatState, CombatError> {
    let mut state = initial_state.clone();
    for input in inputs {
        state = attack(&state, config, input.clone())?.state;
    }
    Ok(state)
}

/// Standard RustRig input model for `combat.attack`.
pub type Input = CombatInput;
/// Standard RustRig state model for `combat.attack`.
pub type State = CombatState;
/// Standard RustRig output/receipt model for `combat.attack`.
pub type Output = CombatOutput;
/// Standard RustRig error model for `combat.attack`.
pub type Error = CombatError;

pub fn default_config() -> CombatConfig {
    CombatConfig::new(u64::MAX, ["slash", "pierce", "blunt", "direct", "basic"])
}

pub fn apply(input: Input, state: State) -> Result<Output, Error> {
    attack(&state, &default_config(), input)
}

pub fn combat_attack(input: Input, state: State) -> Result<Output, Error> {
    apply(input, state)
}

pub fn replay(inputs: &[Input], genesis: State) -> Result<State, Error> {
    let mut state = genesis;
    for input in inputs {
        state = apply(input.clone(), state)?.state;
    }
    Ok(state)
}

pub fn state_root(state: &State) -> String {
    state.state_root()
}

pub fn certified_status() -> &'static str {
    CERTIFICATION_STATUS
}

fn validate(
    state: &CombatState,
    config: &CombatConfig,
    input: &CombatInput,
) -> Result<(), CombatError> {
    let attacker = state
        .entity(&input.attacker_id)
        .ok_or(CombatError::AttackerMissing)?;
    let target = state
        .entity(&input.target_id)
        .ok_or(CombatError::TargetMissing)?;
    if !attacker.alive {
        return Err(CombatError::AttackerDead);
    }
    if !target.alive {
        return Err(CombatError::TargetDead);
    }
    if input.damage == 0 {
        return Err(CombatError::DamageMustBePositive);
    }
    if input.damage > config.max_damage {
        return Err(CombatError::DamageExceedsMax);
    }
    if !config.declares_attack_type(&input.attack_type) {
        return Err(CombatError::AttackTypeNotDeclared);
    }
    Ok(())
}

fn canonical_input(input: &CombatInput) -> String {
    format!(
        "attacker_id={};target_id={};damage={};attack_type={};tick={}",
        input.attacker_id, input.target_id, input.damage, input.attack_type, input.tick
    )
}

fn digest(value: &str) -> String {
    const OFFSET: u128 = 0x6c62_272e_07bb_0142_62b8_2175_6295_c58d;
    const PRIME: u128 = 0x0000_0000_0100_0000_0000_0000_013b;
    let mut hash = OFFSET;
    for byte in value.as_bytes() {
        hash ^= u128::from(*byte);
        hash = hash.wrapping_mul(PRIME);
    }
    format!("ea{:032x}", hash)
}
