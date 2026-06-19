use std::collections::{BTreeMap, BTreeSet};

pub const RUSTRIG_ID: &str = "market.trade";
pub const RUSTRIG_VERSION: &str = "1.0.0";
pub const CERTIFICATION_STATUS: &str = "RUSTRIG MARKET TRADE CERTIFICATION: PASS";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MarketConfig {
    pub declared_trade_types: BTreeSet<String>,
}

impl MarketConfig {
    pub fn new(trade_types: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            declared_trade_types: trade_types.into_iter().map(Into::into).collect(),
        }
    }

    pub fn declares_trade_type(&self, trade_type: &str) -> bool {
        self.declared_trade_types.contains(trade_type)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MarketPlayer {
    pub player_id: String,
    pub balances: BTreeMap<String, u64>,
    pub inventory: BTreeMap<String, u64>,
}

impl MarketPlayer {
    pub fn new(player_id: impl Into<String>) -> Self {
        Self {
            player_id: player_id.into(),
            balances: BTreeMap::new(),
            inventory: BTreeMap::new(),
        }
    }

    pub fn with_balance(mut self, currency_id: impl Into<String>, balance: u64) -> Self {
        self.balances.insert(currency_id.into(), balance);
        self
    }

    pub fn with_item(mut self, item_id: impl Into<String>, quantity: u64) -> Self {
        self.inventory.insert(item_id.into(), quantity);
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TradeInput {
    pub seller_id: String,
    pub buyer_id: String,
    pub item_id: String,
    pub quantity: u64,
    pub unit_price: u64,
    pub currency_id: String,
    pub tick: u64,
    pub trade_type: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TradeOrder {
    pub trade_id: String,
    pub seller_id: String,
    pub buyer_id: String,
    pub item_id: String,
    pub quantity: u64,
    pub unit_price: u64,
    pub currency_id: String,
    pub total_price: u64,
    pub tick: u64,
}

pub type TradeLogEntry = TradeOrder;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TradeReceipt {
    pub receipt_id: String,
    pub rustrig_id: String,
    pub version: String,
    pub seller_id: String,
    pub buyer_id: String,
    pub item_id: String,
    pub quantity: u64,
    pub unit_price: u64,
    pub currency_id: String,
    pub total_price: u64,
    pub seller_item_before: u64,
    pub seller_item_after: u64,
    pub buyer_item_before: u64,
    pub buyer_item_after: u64,
    pub seller_balance_before: u64,
    pub seller_balance_after: u64,
    pub buyer_balance_before: u64,
    pub buyer_balance_after: u64,
    pub tick: u64,
    pub input_hash: String,
    pub pre_state_root: String,
    pub post_state_root: String,
    pub authority: String,
    pub result: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MarketState {
    pub players: BTreeMap<String, MarketPlayer>,
    pub balances: BTreeMap<(String, String), u64>,
    pub inventory: BTreeMap<(String, String), u64>,
    pub orders: BTreeMap<String, TradeOrder>,
    pub trade_log: Vec<TradeLogEntry>,
    pub receipts: BTreeMap<String, TradeReceipt>,
}

impl MarketState {
    pub fn new() -> Self {
        Self {
            players: BTreeMap::new(),
            balances: BTreeMap::new(),
            inventory: BTreeMap::new(),
            orders: BTreeMap::new(),
            trade_log: Vec::new(),
            receipts: BTreeMap::new(),
        }
    }

    pub fn with_player(mut self, player: MarketPlayer) -> Self {
        let player_id = player.player_id.clone();
        for (currency_id, balance) in &player.balances {
            self.balances
                .insert((player_id.clone(), currency_id.clone()), *balance);
        }
        for (item_id, quantity) in &player.inventory {
            self.inventory
                .insert((player_id.clone(), item_id.clone()), *quantity);
        }
        self.players.insert(player_id, player);
        self
    }

    pub fn balance(&self, player_id: &str, currency_id: &str) -> u64 {
        self.balances
            .get(&(player_id.to_owned(), currency_id.to_owned()))
            .copied()
            .unwrap_or(0)
    }

    pub fn item_quantity(&self, player_id: &str, item_id: &str) -> u64 {
        self.inventory
            .get(&(player_id.to_owned(), item_id.to_owned()))
            .copied()
            .unwrap_or(0)
    }

    pub fn contains_item(&self, item_id: &str) -> bool {
        self.inventory.keys().any(|(_, id)| id == item_id)
    }

    pub fn contains_currency(&self, currency_id: &str) -> bool {
        self.balances.keys().any(|(_, id)| id == currency_id)
    }

    pub fn total_item_supply(&self, item_id: &str) -> u64 {
        self.inventory
            .iter()
            .filter(|((_, id), _)| id == item_id)
            .map(|(_, quantity)| *quantity)
            .sum()
    }

    pub fn total_currency_supply(&self, currency_id: &str) -> u64 {
        self.balances
            .iter()
            .filter(|((_, id), _)| id == currency_id)
            .map(|(_, balance)| *balance)
            .sum()
    }

    pub fn state_root(&self) -> String {
        digest(&self.canonical())
    }

    fn canonical(&self) -> String {
        let players = self
            .players
            .iter()
            .map(|(id, player)| {
                let balances = player
                    .balances
                    .iter()
                    .map(|(currency, balance)| format!("{currency}:{balance}"))
                    .collect::<Vec<_>>()
                    .join(",");
                let inventory = player
                    .inventory
                    .iter()
                    .map(|(item, quantity)| format!("{item}:{quantity}"))
                    .collect::<Vec<_>>()
                    .join(",");
                format!("player:{id}:balances=[{balances}]:inventory=[{inventory}]")
            })
            .collect::<Vec<_>>()
            .join("|");
        let balances = self
            .balances
            .iter()
            .map(|((player, currency), balance)| format!("balance:{player}:{currency}:{balance}"))
            .collect::<Vec<_>>()
            .join("|");
        let inventory = self
            .inventory
            .iter()
            .map(|((player, item), quantity)| format!("inventory:{player}:{item}:{quantity}"))
            .collect::<Vec<_>>()
            .join("|");
        let orders = canonical_trades(self.orders.values());
        let trade_log = canonical_trades(self.trade_log.iter());
        let receipts = self
            .receipts
            .iter()
            .map(|(id, receipt)| canonical_receipt(id, receipt))
            .collect::<Vec<_>>()
            .join("|");
        format!(
            "players=[{players}];balances=[{balances}];inventory=[{inventory}];orders=[{orders}];trade_log=[{trade_log}];receipts=[{receipts}]"
        )
    }
}

impl Default for MarketState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TradeOutput {
    pub receipt: TradeReceipt,
    pub state: MarketState,
    pub state_root: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TradeError {
    SellerMissing,
    BuyerMissing,
    SellerEqualsBuyer,
    ItemMissing,
    CurrencyMissing,
    QuantityMustBePositive,
    UnitPriceMustBePositive,
    TotalPriceOverflow,
    InsufficientSellerInventory,
    InsufficientBuyerBalance,
    TradeTypeNotDeclared,
    BalanceOverflow,
    InventoryOverflow,
}

pub fn trade(
    state: &MarketState,
    config: &MarketConfig,
    input: TradeInput,
) -> Result<TradeOutput, TradeError> {
    let total_price = validate(state, config, &input)?;
    let pre_state_root = state.state_root();
    let input_hash = digest(&canonical_input(&input));
    let mut next_state = state.clone();

    let seller_item_before = state.item_quantity(&input.seller_id, &input.item_id);
    let buyer_item_before = state.item_quantity(&input.buyer_id, &input.item_id);
    let seller_balance_before = state.balance(&input.seller_id, &input.currency_id);
    let buyer_balance_before = state.balance(&input.buyer_id, &input.currency_id);
    let seller_item_after = seller_item_before - input.quantity;
    let buyer_item_after = buyer_item_before
        .checked_add(input.quantity)
        .ok_or(TradeError::InventoryOverflow)?;
    let buyer_balance_after = buyer_balance_before - total_price;
    let seller_balance_after = seller_balance_before
        .checked_add(total_price)
        .ok_or(TradeError::BalanceOverflow)?;

    set_item(
        &mut next_state,
        &input.seller_id,
        &input.item_id,
        seller_item_after,
    );
    set_item(
        &mut next_state,
        &input.buyer_id,
        &input.item_id,
        buyer_item_after,
    );
    set_balance(
        &mut next_state,
        &input.buyer_id,
        &input.currency_id,
        buyer_balance_after,
    );
    set_balance(
        &mut next_state,
        &input.seller_id,
        &input.currency_id,
        seller_balance_after,
    );

    let transition_root = next_state.state_root();
    let receipt_id = digest(&format!(
        "{RUSTRIG_ID}|{RUSTRIG_VERSION}|{input_hash}|{pre_state_root}|{transition_root}|{}|{}|{}|{}|{}|{}|{}",
        input.seller_id, input.buyer_id, input.item_id, input.quantity, input.unit_price, input.currency_id, input.tick
    ));
    let order = TradeOrder {
        trade_id: receipt_id.clone(),
        seller_id: input.seller_id.clone(),
        buyer_id: input.buyer_id.clone(),
        item_id: input.item_id.clone(),
        quantity: input.quantity,
        unit_price: input.unit_price,
        currency_id: input.currency_id.clone(),
        total_price,
        tick: input.tick,
    };
    next_state
        .orders
        .insert(order.trade_id.clone(), order.clone());
    next_state.trade_log.push(order);

    let mut receipt = TradeReceipt {
        receipt_id: receipt_id.clone(),
        rustrig_id: RUSTRIG_ID.to_owned(),
        version: RUSTRIG_VERSION.to_owned(),
        seller_id: input.seller_id,
        buyer_id: input.buyer_id,
        item_id: input.item_id,
        quantity: input.quantity,
        unit_price: input.unit_price,
        currency_id: input.currency_id,
        total_price,
        seller_item_before,
        seller_item_after,
        buyer_item_before,
        buyer_item_after,
        seller_balance_before,
        seller_balance_after,
        buyer_balance_before,
        buyer_balance_after,
        tick: input.tick,
        input_hash,
        pre_state_root,
        post_state_root: String::new(),
        authority: "world".to_owned(),
        result: "accepted".to_owned(),
    };
    next_state
        .receipts
        .insert(receipt_id.clone(), receipt.clone());
    let post_state_root = next_state.state_root();
    receipt.post_state_root = post_state_root.clone();
    next_state.receipts.insert(receipt_id, receipt.clone());

    Ok(TradeOutput {
        receipt,
        state: next_state,
        state_root: post_state_root,
    })
}

pub fn replay_with_config(
    initial_state: &MarketState,
    config: &MarketConfig,
    inputs: &[TradeInput],
) -> Result<MarketState, TradeError> {
    let mut state = initial_state.clone();
    for input in inputs {
        state = trade(&state, config, input.clone())?.state;
    }
    Ok(state)
}

/// Standard RustRig input model for `market.trade`.
pub type Input = TradeInput;
/// Standard RustRig state model for `market.trade`.
pub type State = MarketState;
/// Standard RustRig output/receipt model for `market.trade`.
pub type Output = TradeOutput;
/// Standard RustRig error model for `market.trade`.
pub type Error = TradeError;

pub fn default_config() -> MarketConfig {
    MarketConfig::new(["direct", "fixed_price"])
}

pub fn apply(input: Input, state: State) -> Result<Output, Error> {
    trade(&state, &default_config(), input)
}

pub fn market_trade(input: Input, state: State) -> Result<Output, Error> {
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
    state: &MarketState,
    config: &MarketConfig,
    input: &TradeInput,
) -> Result<u64, TradeError> {
    if !state.players.contains_key(&input.seller_id) {
        return Err(TradeError::SellerMissing);
    }
    if !state.players.contains_key(&input.buyer_id) {
        return Err(TradeError::BuyerMissing);
    }
    if input.seller_id == input.buyer_id {
        return Err(TradeError::SellerEqualsBuyer);
    }
    if !state.contains_item(&input.item_id) {
        return Err(TradeError::ItemMissing);
    }
    if !state.contains_currency(&input.currency_id) {
        return Err(TradeError::CurrencyMissing);
    }
    if input.quantity == 0 {
        return Err(TradeError::QuantityMustBePositive);
    }
    if input.unit_price == 0 {
        return Err(TradeError::UnitPriceMustBePositive);
    }
    let total_price = input
        .quantity
        .checked_mul(input.unit_price)
        .ok_or(TradeError::TotalPriceOverflow)?;
    if state.item_quantity(&input.seller_id, &input.item_id) < input.quantity {
        return Err(TradeError::InsufficientSellerInventory);
    }
    if state.balance(&input.buyer_id, &input.currency_id) < total_price {
        return Err(TradeError::InsufficientBuyerBalance);
    }
    if !config.declares_trade_type(&input.trade_type) {
        return Err(TradeError::TradeTypeNotDeclared);
    }
    Ok(total_price)
}

fn set_item(state: &mut MarketState, player_id: &str, item_id: &str, quantity: u64) {
    state
        .inventory
        .insert((player_id.to_owned(), item_id.to_owned()), quantity);
    state
        .players
        .get_mut(player_id)
        .expect("player existence validated before mutation")
        .inventory
        .insert(item_id.to_owned(), quantity);
}

fn set_balance(state: &mut MarketState, player_id: &str, currency_id: &str, balance: u64) {
    state
        .balances
        .insert((player_id.to_owned(), currency_id.to_owned()), balance);
    state
        .players
        .get_mut(player_id)
        .expect("player existence validated before mutation")
        .balances
        .insert(currency_id.to_owned(), balance);
}

fn canonical_input(input: &TradeInput) -> String {
    format!(
        "seller_id={};buyer_id={};item_id={};quantity={};unit_price={};currency_id={};tick={};trade_type={}",
        input.seller_id,
        input.buyer_id,
        input.item_id,
        input.quantity,
        input.unit_price,
        input.currency_id,
        input.tick,
        input.trade_type
    )
}

fn canonical_trades<'a>(trades: impl Iterator<Item = &'a TradeOrder>) -> String {
    trades
        .map(|trade| {
            format!(
                "trade:{}:{}:{}:{}:{}:{}:{}:{}:{}",
                trade.trade_id,
                trade.seller_id,
                trade.buyer_id,
                trade.item_id,
                trade.quantity,
                trade.unit_price,
                trade.currency_id,
                trade.total_price,
                trade.tick
            )
        })
        .collect::<Vec<_>>()
        .join("|")
}

fn canonical_receipt(id: &str, receipt: &TradeReceipt) -> String {
    format!(
        "receipt:{id}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}",
        receipt.rustrig_id,
        receipt.version,
        receipt.seller_id,
        receipt.buyer_id,
        receipt.item_id,
        receipt.quantity,
        receipt.unit_price,
        receipt.currency_id,
        receipt.total_price,
        receipt.seller_item_before,
        receipt.seller_item_after,
        receipt.buyer_item_before,
        receipt.buyer_item_after,
        receipt.seller_balance_before,
        receipt.seller_balance_after,
        receipt.buyer_balance_before,
        receipt.buyer_balance_after,
        receipt.tick,
        receipt.input_hash,
        receipt.pre_state_root,
        receipt.post_state_root,
        receipt.result
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
