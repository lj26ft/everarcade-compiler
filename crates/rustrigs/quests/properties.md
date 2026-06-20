# Property Targets: quest.start


## Property: Declared Mutation Safety

Given:
Valid quest.start pre-state satisfying declared state domains.

When:
`quest.start()` executes under declared mutation authority.

Then:
quest.start must reject invalid preconditions before mutating canonical state.


## Property: State Integrity

Given:
Valid quest.start pre-state satisfying declared state domains.

When:
`quest.start()` executes under declared mutation authority.

Then:
quest.start must preserve unrelated state domains.


## Property: Continuity Trace

Given:
Valid quest.start pre-state satisfying declared state domains.

When:
`quest.start()` executes under declared mutation authority.

Then:
quest.start must emit replay-stable evidence for verifier review.

# Property Targets: quest.complete


## Property: Declared Mutation Safety

Given:
Valid quest.complete pre-state satisfying declared state domains.

When:
`quest.complete()` executes under declared mutation authority.

Then:
quest.complete must reject invalid preconditions before mutating canonical state.


## Property: State Integrity

Given:
Valid quest.complete pre-state satisfying declared state domains.

When:
`quest.complete()` executes under declared mutation authority.

Then:
quest.complete must preserve unrelated state domains.


## Property: Continuity Trace

Given:
Valid quest.complete pre-state satisfying declared state domains.

When:
`quest.complete()` executes under declared mutation authority.

Then:
quest.complete must emit replay-stable evidence for verifier review.
