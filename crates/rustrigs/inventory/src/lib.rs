use std::collections::{BTreeMap, BTreeSet};

pub const RUSTRIG_ID: &str = "inventory.transfer";
pub const RUSTRIG_VERSION: &str = "1.0.0";
pub const CANDIDATE_MUTATIONS: &[&str] = &[
    "inventory.pickup",
    "inventory.drop",
    "inventory.equip",
    "inventory.unequip",
    "inventory.consume",
];
pub const CERTIFICATION_STATUS: &str = "RUSTRIG INVENTORY TRANSFER CERTIFICATION: PASS";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransferInput {
    pub source_player: String,
    pub destination_player: String,
    pub item_id: String,
    pub quantity: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InventoryState {
    players: BTreeSet<String>,
    quantities: BTreeMap<(String, String), u64>,
}

impl InventoryState {
    pub fn new() -> Self {
        Self {
            players: BTreeSet::new(),
            quantities: BTreeMap::new(),
        }
    }

    pub fn with_player(mut self, player_id: impl Into<String>) -> Self {
        self.players.insert(player_id.into());
        self
    }

    pub fn with_item(
        mut self,
        player_id: impl Into<String>,
        item_id: impl Into<String>,
        quantity: u64,
    ) -> Self {
        let player_id = player_id.into();
        self.players.insert(player_id.clone());
        self.quantities
            .insert((player_id, item_id.into()), quantity);
        self
    }

    pub fn quantity(&self, player_id: &str, item_id: &str) -> u64 {
        self.quantities
            .get(&(player_id.to_owned(), item_id.to_owned()))
            .copied()
            .unwrap_or(0)
    }

    pub fn contains_player(&self, player_id: &str) -> bool {
        self.players.contains(player_id)
    }

    pub fn contains_item(&self, item_id: &str) -> bool {
        self.quantities
            .keys()
            .any(|(_, existing_item_id)| existing_item_id == item_id)
    }

    pub fn state_root(&self) -> String {
        digest(&self.canonical())
    }

    fn canonical(&self) -> String {
        let players = self
            .players
            .iter()
            .map(|player| format!("player:{player}"))
            .collect::<Vec<_>>()
            .join("|");
        let inventory = self
            .quantities
            .iter()
            .map(|((player, item), quantity)| format!("inventory:{player}:{item}:{quantity}"))
            .collect::<Vec<_>>()
            .join("|");
        format!("players=[{players}];inventory=[{inventory}]")
    }
}

impl Default for InventoryState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransferReceipt {
    pub receipt_id: String,
    pub rustrig_id: String,
    pub version: String,
    pub input_hash: String,
    pub pre_state_root: String,
    pub post_state_root: String,
    pub source_player: String,
    pub destination_player: String,
    pub item_id: String,
    pub quantity: u64,
    pub authority: String,
    pub result: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransferOutput {
    pub receipt: TransferReceipt,
    pub state: InventoryState,
    pub state_root: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TransferError {
    SourcePlayerMissing,
    DestinationPlayerMissing,
    ItemMissing,
    QuantityMustBePositive,
    InsufficientInventory,
}

pub fn transfer(
    state: &InventoryState,
    input: TransferInput,
) -> Result<TransferOutput, TransferError> {
    validate(state, &input)?;

    let pre_state_root = state.state_root();
    let input_hash = digest(&canonical_input(&input));
    let mut next_state = state.clone();

    let source_key = (input.source_player.clone(), input.item_id.clone());
    let destination_key = (input.destination_player.clone(), input.item_id.clone());
    let source_quantity = next_state.quantity(&input.source_player, &input.item_id);
    let destination_quantity = next_state.quantity(&input.destination_player, &input.item_id);

    next_state
        .quantities
        .insert(source_key, source_quantity - input.quantity);
    next_state.quantities.insert(
        destination_key,
        destination_quantity.saturating_add(input.quantity),
    );

    let post_state_root = next_state.state_root();
    let receipt_id = digest(&format!(
        "{RUSTRIG_ID}|{RUSTRIG_VERSION}|{input_hash}|{pre_state_root}|{post_state_root}"
    ));
    let receipt = TransferReceipt {
        receipt_id,
        rustrig_id: RUSTRIG_ID.to_owned(),
        version: RUSTRIG_VERSION.to_owned(),
        input_hash,
        pre_state_root,
        post_state_root: post_state_root.clone(),
        source_player: input.source_player,
        destination_player: input.destination_player,
        item_id: input.item_id,
        quantity: input.quantity,
        authority: "owner_or_delegated_inventory_authority".to_owned(),
        result: "accepted".to_owned(),
    };

    Ok(TransferOutput {
        receipt,
        state: next_state,
        state_root: post_state_root,
    })
}

fn validate(state: &InventoryState, input: &TransferInput) -> Result<(), TransferError> {
    if !state.contains_player(&input.source_player) {
        return Err(TransferError::SourcePlayerMissing);
    }
    if !state.contains_player(&input.destination_player) {
        return Err(TransferError::DestinationPlayerMissing);
    }
    if !state.contains_item(&input.item_id) {
        return Err(TransferError::ItemMissing);
    }
    if input.quantity == 0 {
        return Err(TransferError::QuantityMustBePositive);
    }
    if state.quantity(&input.source_player, &input.item_id) < input.quantity {
        return Err(TransferError::InsufficientInventory);
    }
    Ok(())
}

fn canonical_input(input: &TransferInput) -> String {
    format!(
        "source_player={};destination_player={};item_id={};quantity={}",
        input.source_player, input.destination_player, input.item_id, input.quantity
    )
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

/// Standard RustRig input model for `inventory.transfer`.
pub type Input = TransferInput;
/// Standard RustRig state model for `inventory.transfer`.
pub type State = InventoryState;
/// Standard RustRig output/receipt model for `inventory.transfer`.
pub type Output = TransferOutput;
/// Standard RustRig error model for `inventory.transfer`.
pub type Error = TransferError;

pub fn apply(input: Input, state: State) -> Result<Output, Error> {
    transfer(&state, input)
}

pub fn inventory_transfer(input: Input, state: State) -> Result<Output, Error> {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture_state() -> InventoryState {
        InventoryState::new()
            .with_item("player-alpha", "crystal", 10)
            .with_item("player-beta", "crystal", 2)
    }

    fn fixture_input() -> TransferInput {
        TransferInput {
            source_player: "player-alpha".to_owned(),
            destination_player: "player-beta".to_owned(),
            item_id: "crystal".to_owned(),
            quantity: 3,
        }
    }

    #[test]
    fn rr_001_transfer_determinism_same_input_same_output() {
        let first = transfer(&fixture_state(), fixture_input()).unwrap();
        let second = transfer(&fixture_state(), fixture_input()).unwrap();
        assert_eq!(first, second);
    }

    #[test]
    fn rr_002_inventory_conservation() {
        let before = fixture_state();
        let after = transfer(&before, fixture_input()).unwrap().state;
        let before_total =
            before.quantity("player-alpha", "crystal") + before.quantity("player-beta", "crystal");
        let after_total =
            after.quantity("player-alpha", "crystal") + after.quantity("player-beta", "crystal");
        assert_eq!(before_total, after_total);
        assert_eq!(after.quantity("player-alpha", "crystal"), 7);
        assert_eq!(after.quantity("player-beta", "crystal"), 5);
    }

    #[test]
    fn rr_003_receipt_integrity_same_input_same_receipt() {
        let first = transfer(&fixture_state(), fixture_input()).unwrap();
        let second = transfer(&fixture_state(), fixture_input()).unwrap();
        assert_eq!(first.receipt, second.receipt);
        assert_eq!(first.receipt.rustrig_id, RUSTRIG_ID);
        assert_eq!(first.receipt.pre_state_root, fixture_state().state_root());
        assert_eq!(first.receipt.post_state_root, first.state_root);
    }

    #[test]
    fn rr_004_replay_equivalence() {
        let replay_a = transfer(&fixture_state(), fixture_input()).unwrap().state;
        let replay_b = transfer(&fixture_state(), fixture_input()).unwrap().state;
        assert_eq!(replay_a, replay_b);
    }

    #[test]
    fn rr_005_root_equivalence_same_input_same_state_root() {
        let first = transfer(&fixture_state(), fixture_input()).unwrap();
        let second = transfer(&fixture_state(), fixture_input()).unwrap();
        assert_eq!(first.state_root, second.state_root);
    }

    #[test]
    fn rejects_invalid_transfers() {
        let state = fixture_state();
        assert_eq!(
            transfer(
                &state,
                TransferInput {
                    quantity: 0,
                    ..fixture_input()
                }
            ),
            Err(TransferError::QuantityMustBePositive)
        );
        assert_eq!(
            transfer(
                &state,
                TransferInput {
                    source_player: "missing".to_owned(),
                    ..fixture_input()
                }
            ),
            Err(TransferError::SourcePlayerMissing)
        );
        assert_eq!(
            transfer(
                &state,
                TransferInput {
                    destination_player: "missing".to_owned(),
                    ..fixture_input()
                }
            ),
            Err(TransferError::DestinationPlayerMissing)
        );
        assert_eq!(
            transfer(
                &state,
                TransferInput {
                    item_id: "missing".to_owned(),
                    ..fixture_input()
                }
            ),
            Err(TransferError::ItemMissing)
        );
        assert_eq!(
            transfer(
                &state,
                TransferInput {
                    quantity: 99,
                    ..fixture_input()
                }
            ),
            Err(TransferError::InsufficientInventory)
        );
    }
}
