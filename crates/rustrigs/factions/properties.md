# Property Targets: faction.create


## Property: Declared Mutation Safety

Given:
Valid faction.create pre-state satisfying declared state domains.

When:
`faction.create()` executes under declared mutation authority.

Then:
faction.create must reject invalid preconditions before mutating canonical state.


## Property: State Integrity

Given:
Valid faction.create pre-state satisfying declared state domains.

When:
`faction.create()` executes under declared mutation authority.

Then:
faction.create must preserve unrelated state domains.


## Property: Continuity Trace

Given:
Valid faction.create pre-state satisfying declared state domains.

When:
`faction.create()` executes under declared mutation authority.

Then:
faction.create must emit replay-stable evidence for verifier review.

# Property Targets: faction.join


## Property: Declared Mutation Safety

Given:
Valid faction.join pre-state satisfying declared state domains.

When:
`faction.join()` executes under declared mutation authority.

Then:
faction.join must reject invalid preconditions before mutating canonical state.


## Property: State Integrity

Given:
Valid faction.join pre-state satisfying declared state domains.

When:
`faction.join()` executes under declared mutation authority.

Then:
faction.join must preserve unrelated state domains.


## Property: Continuity Trace

Given:
Valid faction.join pre-state satisfying declared state domains.

When:
`faction.join()` executes under declared mutation authority.

Then:
faction.join must emit replay-stable evidence for verifier review.
