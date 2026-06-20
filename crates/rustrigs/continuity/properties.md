# Property Targets: continuity.advance


## Property: Declared Mutation Safety

Given:
Valid continuity.advance pre-state satisfying declared state domains.

When:
`continuity.advance()` executes under declared mutation authority.

Then:
continuity.advance must reject invalid preconditions before mutating canonical state.


## Property: State Integrity

Given:
Valid continuity.advance pre-state satisfying declared state domains.

When:
`continuity.advance()` executes under declared mutation authority.

Then:
continuity.advance must preserve unrelated state domains.


## Property: Continuity Trace

Given:
Valid continuity.advance pre-state satisfying declared state domains.

When:
`continuity.advance()` executes under declared mutation authority.

Then:
continuity.advance must emit replay-stable evidence for verifier review.

# Property Targets: continuity.record_event


## Property: Declared Mutation Safety

Given:
Valid continuity.record_event pre-state satisfying declared state domains.

When:
`continuity.record_event()` executes under declared mutation authority.

Then:
continuity.record_event must reject invalid preconditions before mutating canonical state.


## Property: State Integrity

Given:
Valid continuity.record_event pre-state satisfying declared state domains.

When:
`continuity.record_event()` executes under declared mutation authority.

Then:
continuity.record_event must preserve unrelated state domains.


## Property: Continuity Trace

Given:
Valid continuity.record_event pre-state satisfying declared state domains.

When:
`continuity.record_event()` executes under declared mutation authority.

Then:
continuity.record_event must emit replay-stable evidence for verifier review.
