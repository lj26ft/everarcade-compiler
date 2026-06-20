# Property Targets: identity.join


## Property: Declared Mutation Safety

Given:
Valid identity.join pre-state satisfying declared state domains.

When:
`identity.join()` executes under declared mutation authority.

Then:
identity.join must reject invalid preconditions before mutating canonical state.


## Property: State Integrity

Given:
Valid identity.join pre-state satisfying declared state domains.

When:
`identity.join()` executes under declared mutation authority.

Then:
identity.join must preserve unrelated state domains.


## Property: Continuity Trace

Given:
Valid identity.join pre-state satisfying declared state domains.

When:
`identity.join()` executes under declared mutation authority.

Then:
identity.join must emit replay-stable evidence for verifier review.

# Property Targets: identity.leave


## Property: Declared Mutation Safety

Given:
Valid identity.leave pre-state satisfying declared state domains.

When:
`identity.leave()` executes under declared mutation authority.

Then:
identity.leave must reject invalid preconditions before mutating canonical state.


## Property: State Integrity

Given:
Valid identity.leave pre-state satisfying declared state domains.

When:
`identity.leave()` executes under declared mutation authority.

Then:
identity.leave must preserve unrelated state domains.


## Property: Continuity Trace

Given:
Valid identity.leave pre-state satisfying declared state domains.

When:
`identity.leave()` executes under declared mutation authority.

Then:
identity.leave must emit replay-stable evidence for verifier review.
