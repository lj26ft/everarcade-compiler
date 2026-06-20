# Property Targets: inventory.transfer


## Property: Conservation

Given:
Valid inventory.transfer pre-state satisfying declared state domains.

When:
`inventory.transfer()` executes under declared mutation authority.

Then:
Transfers cannot create or destroy items.


## Property: No Overdraw

Given:
Valid inventory.transfer pre-state satisfying declared state domains.

When:
`inventory.transfer()` executes under declared mutation authority.

Then:
Sender inventory cannot become negative.


## Property: Owner Authorization

Given:
Valid inventory.transfer pre-state satisfying declared state domains.

When:
`inventory.transfer()` executes under declared mutation authority.

Then:
Only the current owner may initiate transfer.
