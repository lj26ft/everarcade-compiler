# Property Targets: crafting.craft


## Property: Declared Mutation Safety

Given:
Valid crafting.craft pre-state satisfying declared state domains.

When:
`crafting.craft()` executes under declared mutation authority.

Then:
crafting.craft must reject invalid preconditions before mutating canonical state.


## Property: State Integrity

Given:
Valid crafting.craft pre-state satisfying declared state domains.

When:
`crafting.craft()` executes under declared mutation authority.

Then:
crafting.craft must preserve unrelated state domains.


## Property: Continuity Trace

Given:
Valid crafting.craft pre-state satisfying declared state domains.

When:
`crafting.craft()` executes under declared mutation authority.

Then:
crafting.craft must emit replay-stable evidence for verifier review.
