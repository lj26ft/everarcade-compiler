# Property Targets: combat.attack


## Property: Damage Bounds

Given:
Valid combat.attack pre-state satisfying declared state domains.

When:
`combat.attack()` executes under declared mutation authority.

Then:
Attack resolution cannot apply negative damage or exceed declared attack bounds.


## Property: Target Integrity

Given:
Valid combat.attack pre-state satisfying declared state domains.

When:
`combat.attack()` executes under declared mutation authority.

Then:
Only the declared target combat state may be mutated by the attack.


## Property: Turn Ordering

Given:
Valid combat.attack pre-state satisfying declared state domains.

When:
`combat.attack()` executes under declared mutation authority.

Then:
Attack effects follow deterministic action ordering.
