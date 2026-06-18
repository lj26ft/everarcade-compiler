# market.trade() RustRig

`market.trade()` is the certified deterministic economic mutation for player-to-player marketplace exchange in EverArcade worlds.

## Mutation declaration

World Contracts declare the primitive as a certified mutation:

```toml
[rustrigs.market.trade]
crate = "everarcade-rustrig-market"
certification = "PASS"
authority = "world"
receipt = "TradeReceipt"
```

The certified mutation surface reads players, inventory, and economy state, then writes inventory, economy, and receipts.

## Authority rules

`market.trade()` uses World Contract authority. The World Contract must declare the supported deterministic trade type, and the RustRig rejects any trade type not present in that declaration.

## Deterministic integer economics

All economic values are integer units. The RustRig uses no floating point, exchange-rate lookup, timestamps, randomness, platform-dependent decimal math, hidden fees, or dynamic pricing. The total price is always:

```text
total_price = quantity * unit_price
```

The multiplication is checked, and overflow is rejected deterministically.

## State transition

For a valid trade:

```text
seller.inventory[item_id] -= quantity
buyer.inventory[item_id] += quantity
buyer.balance[currency_id] -= total_price
seller.balance[currency_id] += total_price
```

The state model contains players, balances, inventory, orders, trade log, and receipts. Canonical roots are derived from sorted maps plus ordered trade log entries.

## Conservation rules

The transition preserves total item supply and total currency supply. No trade mints, burns, destroys, fees, or creates assets.

## Receipt generation

`TradeReceipt` records the deterministic before and after values for seller inventory, buyer inventory, seller balance, buyer balance, total price, tick, input hash, pre-state root, and post-state root.

## Replay behavior

Replay applies the same ordered `TradeInput` values from the same initial `MarketState`. The resulting state, receipts, and roots are identical to direct execution.

## Certification requirements

The certified implementation covers:

- RR-MARKET-001 Trade Determinism
- RR-MARKET-002 Item Conservation
- RR-MARKET-003 Currency Conservation
- RR-MARKET-004 Balance Integrity
- RR-MARKET-005 Inventory Integrity
- RR-MARKET-006 Receipt Integrity
- RR-MARKET-007 Replay Equivalence
- RR-MARKET-008 Root Equivalence
- RR-MARKET-009 Overflow Safety

Certification output:

```text
RUSTRIG MARKET TRADE CERTIFICATION: PASS
```

## Future deterministic economic RustRigs

Future RustRigs must inherit the certification model established by `market.trade()`:

- `market.list()`
- `market.cancel()`
- `market.bid()`
- `market.accept_bid()`
- `market.auction_start()`
- `market.auction_settle()`
- `market.royalty_distribute()`
- `market.fee_collect()`
- `economy.mint()`
- `economy.burn()`
