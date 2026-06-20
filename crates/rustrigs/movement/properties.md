# Property Targets: movement.move


## Property: Declared Mutation Safety

Given:
Valid movement.move pre-state satisfying declared state domains.

When:
`movement.move()` executes under declared mutation authority.

Then:
movement.move must reject invalid preconditions before mutating canonical state.


## Property: State Integrity

Given:
Valid movement.move pre-state satisfying declared state domains.

When:
`movement.move()` executes under declared mutation authority.

Then:
movement.move must preserve unrelated state domains.


## Property: Continuity Trace

Given:
Valid movement.move pre-state satisfying declared state domains.

When:
`movement.move()` executes under declared mutation authority.

Then:
movement.move must emit replay-stable evidence for verifier review.
