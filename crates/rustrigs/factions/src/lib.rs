use std::collections::BTreeMap;

pub const CERTIFICATION_STATUS: &str = "CANDIDATE";

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct State {
    pub facts: BTreeMap<String, String>,
    pub history: Vec<Receipt>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Input {
    pub mutation: &'static str,
    pub actor_id: String,
    pub subject_id: String,
    pub quantity: u64,
    pub tick: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Receipt {
    pub mutation: &'static str,
    pub input_hash: String,
    pub pre_state_root: String,
    pub post_state_root: String,
    pub maturity: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Output {
    pub state: State,
    pub receipt: Receipt,
    pub state_root: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    EmptyActor,
    EmptySubject,
    MutationMismatch,
}

pub fn apply(input: Input, state: State) -> Result<Output, Error> {
    apply_named("faction.create", input, state)
}

pub fn replay(inputs: &[Input], genesis: State) -> Result<State, Error> {
    let mut state = genesis;
    for input in inputs {
        state = apply(input.clone(), state)?.state;
    }
    Ok(state)
}

pub fn state_root(state: &State) -> String {
    digest(&canonical_state(state))
}

pub fn certified_status() -> &'static str {
    CERTIFICATION_STATUS
}

fn apply_named(expected: &'static str, input: Input, state: State) -> Result<Output, Error> {
    if input.actor_id.is_empty() {
        return Err(Error::EmptyActor);
    }
    if input.subject_id.is_empty() {
        return Err(Error::EmptySubject);
    }
    if input.mutation != expected {
        return Err(Error::MutationMismatch);
    }
    let pre_state_root = state_root(&state);
    let input_hash = digest(&format!(
        "{}|{}|{}|{}|{}",
        input.mutation, input.actor_id, input.subject_id, input.quantity, input.tick
    ));
    let mut next = state;
    next.facts.insert(
        format!("{}:{}", input.mutation, input.subject_id),
        format!(
            "actor={};quantity={};tick={}",
            input.actor_id, input.quantity, input.tick
        ),
    );
    let provisional = state_root(&next);
    let receipt = Receipt {
        mutation: expected,
        input_hash,
        pre_state_root,
        post_state_root: provisional,
        maturity: CERTIFICATION_STATUS,
    };
    next.history.push(receipt.clone());
    let post_state_root = state_root(&next);
    let receipt = Receipt {
        post_state_root: post_state_root.clone(),
        ..receipt
    };
    if let Some(last) = next.history.last_mut() {
        *last = receipt.clone();
    }
    Ok(Output {
        state: next,
        receipt,
        state_root: post_state_root,
    })
}

pub fn faction_create(input: Input, state: State) -> Result<Output, Error> {
    apply_named("faction.create", input, state)
}
pub fn faction_join(input: Input, state: State) -> Result<Output, Error> {
    apply_named("faction.join", input, state)
}
pub fn faction_leave(input: Input, state: State) -> Result<Output, Error> {
    apply_named("faction.leave", input, state)
}

fn canonical_state(state: &State) -> String {
    let facts = state
        .facts
        .iter()
        .map(|(k, v)| format!("{k}={v}"))
        .collect::<Vec<_>>()
        .join("|");
    let history = state
        .history
        .iter()
        .map(|r| format!("{}:{}:{}", r.mutation, r.input_hash, r.post_state_root))
        .collect::<Vec<_>>()
        .join("|");
    format!("facts=[{facts}];history=[{history}]")
}

fn digest(value: &str) -> String {
    const OFFSET: u128 = 0x6c62_272e_07bb_0142_62b8_2175_6295_c58d;
    const PRIME: u128 = 0x0000_0000_0100_0000_0000_0000_0000_013b;
    let mut hash = OFFSET;
    for byte in value.as_bytes() {
        hash ^= u128::from(*byte);
        hash = hash.wrapping_mul(PRIME);
    }
    format!("ea{:032x}", hash)
}
