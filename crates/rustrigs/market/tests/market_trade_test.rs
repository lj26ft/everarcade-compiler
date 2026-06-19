use everarcade_rustrig_market::{
    replay_with_config as replay, trade, MarketConfig, MarketPlayer, MarketState, TradeError,
    TradeInput, RUSTRIG_ID,
};

fn config() -> MarketConfig {
    MarketConfig::new(["direct"])
}

fn fixture_state() -> MarketState {
    MarketState::new()
        .with_player(
            MarketPlayer::new("seller")
                .with_item("sword", 5)
                .with_balance("gold", 25),
        )
        .with_player(
            MarketPlayer::new("buyer")
                .with_item("sword", 1)
                .with_balance("gold", 200),
        )
}

fn fixture_input() -> TradeInput {
    TradeInput {
        seller_id: "seller".to_owned(),
        buyer_id: "buyer".to_owned(),
        item_id: "sword".to_owned(),
        quantity: 2,
        unit_price: 50,
        currency_id: "gold".to_owned(),
        tick: 7,
        trade_type: "direct".to_owned(),
    }
}

#[test]
fn valid_trade_updates_inventory_balances_log_and_receipt() {
    let before = fixture_state();
    let output = trade(&before, &config(), fixture_input()).unwrap();

    assert_eq!(output.state.item_quantity("seller", "sword"), 3);
    assert_eq!(output.state.item_quantity("buyer", "sword"), 3);
    assert_eq!(output.state.balance("seller", "gold"), 125);
    assert_eq!(output.state.balance("buyer", "gold"), 100);
    assert_eq!(output.receipt.rustrig_id, RUSTRIG_ID);
    assert_eq!(output.receipt.total_price, 100);
    assert_eq!(output.state.trade_log.len(), 1);
    assert_eq!(output.state.orders.len(), 1);
    assert_eq!(output.state.receipts.len(), 1);
}

#[test]
fn rejects_seller_missing() {
    let err = trade(
        &fixture_state(),
        &config(),
        TradeInput {
            seller_id: "missing".to_owned(),
            ..fixture_input()
        },
    )
    .unwrap_err();
    assert_eq!(err, TradeError::SellerMissing);
}

#[test]
fn rejects_buyer_missing() {
    let err = trade(
        &fixture_state(),
        &config(),
        TradeInput {
            buyer_id: "missing".to_owned(),
            ..fixture_input()
        },
    )
    .unwrap_err();
    assert_eq!(err, TradeError::BuyerMissing);
}

#[test]
fn rejects_seller_equals_buyer() {
    let err = trade(
        &fixture_state(),
        &config(),
        TradeInput {
            buyer_id: "seller".to_owned(),
            ..fixture_input()
        },
    )
    .unwrap_err();
    assert_eq!(err, TradeError::SellerEqualsBuyer);
}

#[test]
fn rejects_item_missing() {
    let err = trade(
        &fixture_state(),
        &config(),
        TradeInput {
            item_id: "missing".to_owned(),
            ..fixture_input()
        },
    )
    .unwrap_err();
    assert_eq!(err, TradeError::ItemMissing);
}

#[test]
fn rejects_currency_missing() {
    let err = trade(
        &fixture_state(),
        &config(),
        TradeInput {
            currency_id: "gems".to_owned(),
            ..fixture_input()
        },
    )
    .unwrap_err();
    assert_eq!(err, TradeError::CurrencyMissing);
}

#[test]
fn rejects_zero_quantity() {
    let err = trade(
        &fixture_state(),
        &config(),
        TradeInput {
            quantity: 0,
            ..fixture_input()
        },
    )
    .unwrap_err();
    assert_eq!(err, TradeError::QuantityMustBePositive);
}

#[test]
fn rejects_zero_unit_price() {
    let err = trade(
        &fixture_state(),
        &config(),
        TradeInput {
            unit_price: 0,
            ..fixture_input()
        },
    )
    .unwrap_err();
    assert_eq!(err, TradeError::UnitPriceMustBePositive);
}

#[test]
fn rejects_insufficient_seller_inventory() {
    let err = trade(
        &fixture_state(),
        &config(),
        TradeInput {
            quantity: 6,
            ..fixture_input()
        },
    )
    .unwrap_err();
    assert_eq!(err, TradeError::InsufficientSellerInventory);
}

#[test]
fn rejects_insufficient_buyer_balance() {
    let err = trade(
        &fixture_state(),
        &config(),
        TradeInput {
            unit_price: 101,
            ..fixture_input()
        },
    )
    .unwrap_err();
    assert_eq!(err, TradeError::InsufficientBuyerBalance);
}

#[test]
fn rejects_integer_overflow() {
    let err = trade(
        &fixture_state(),
        &config(),
        TradeInput {
            quantity: u64::MAX,
            unit_price: 2,
            ..fixture_input()
        },
    )
    .unwrap_err();
    assert_eq!(err, TradeError::TotalPriceOverflow);
}

#[test]
fn rejects_undeclared_trade_type() {
    let err = trade(
        &fixture_state(),
        &config(),
        TradeInput {
            trade_type: "auction".to_owned(),
            ..fixture_input()
        },
    )
    .unwrap_err();
    assert_eq!(err, TradeError::TradeTypeNotDeclared);
}

#[test]
fn item_conservation() {
    let before = fixture_state();
    let output = trade(&before, &config(), fixture_input()).unwrap();
    assert_eq!(
        before.total_item_supply("sword"),
        output.state.total_item_supply("sword")
    );
}

#[test]
fn currency_conservation() {
    let before = fixture_state();
    let output = trade(&before, &config(), fixture_input()).unwrap();
    assert_eq!(
        before.total_currency_supply("gold"),
        output.state.total_currency_supply("gold")
    );
}

#[test]
fn receipt_integrity() {
    let output = trade(&fixture_state(), &config(), fixture_input()).unwrap();
    let receipt = output.receipt;
    assert_eq!(receipt.seller_item_before, 5);
    assert_eq!(receipt.seller_item_after, 3);
    assert_eq!(receipt.buyer_item_before, 1);
    assert_eq!(receipt.buyer_item_after, 3);
    assert_eq!(receipt.seller_balance_before, 25);
    assert_eq!(receipt.seller_balance_after, 125);
    assert_eq!(receipt.buyer_balance_before, 200);
    assert_eq!(receipt.buyer_balance_after, 100);
    assert_eq!(receipt.post_state_root, output.state_root);
}

#[test]
fn replay_equivalence() {
    let initial = fixture_state();
    let input = fixture_input();
    let direct = trade(&initial, &config(), input.clone()).unwrap().state;
    let replayed = replay(&initial, &config(), &[input]).unwrap();
    assert_eq!(direct, replayed);
}

#[test]
fn root_equivalence_and_determinism() {
    let first = trade(&fixture_state(), &config(), fixture_input()).unwrap();
    let second = trade(&fixture_state(), &config(), fixture_input()).unwrap();
    assert_eq!(first, second);
    assert_eq!(first.state_root, second.state_root);
    assert_eq!(first.receipt, second.receipt);
}
