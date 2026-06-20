# Property Targets: market.trade


## Property: Asset Conservation

Given:
Valid market.trade pre-state satisfying declared state domains.

When:
`market.trade()` executes under declared mutation authority.

Then:
Trades cannot create or destroy escrowed assets.


## Property: Price Integrity

Given:
Valid market.trade pre-state satisfying declared state domains.

When:
`market.trade()` executes under declared mutation authority.

Then:
Settlement must match the accepted price and quantity.


## Property: Counterparty Authorization

Given:
Valid market.trade pre-state satisfying declared state domains.

When:
`market.trade()` executes under declared mutation authority.

Then:
Only authorized counterparties may settle the trade.
