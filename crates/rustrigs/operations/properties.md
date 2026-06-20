# Property Targets: operation.migrate


## Property: Declared Mutation Safety

Given:
Valid operation.migrate pre-state satisfying declared state domains.

When:
`operation.migrate()` executes under declared mutation authority.

Then:
operation.migrate must reject invalid preconditions before mutating canonical state.


## Property: State Integrity

Given:
Valid operation.migrate pre-state satisfying declared state domains.

When:
`operation.migrate()` executes under declared mutation authority.

Then:
operation.migrate must preserve unrelated state domains.


## Property: Continuity Trace

Given:
Valid operation.migrate pre-state satisfying declared state domains.

When:
`operation.migrate()` executes under declared mutation authority.

Then:
operation.migrate must emit replay-stable evidence for verifier review.

# Property Targets: operation.restore


## Property: Declared Mutation Safety

Given:
Valid operation.restore pre-state satisfying declared state domains.

When:
`operation.restore()` executes under declared mutation authority.

Then:
operation.restore must reject invalid preconditions before mutating canonical state.


## Property: State Integrity

Given:
Valid operation.restore pre-state satisfying declared state domains.

When:
`operation.restore()` executes under declared mutation authority.

Then:
operation.restore must preserve unrelated state domains.


## Property: Continuity Trace

Given:
Valid operation.restore pre-state satisfying declared state domains.

When:
`operation.restore()` executes under declared mutation authority.

Then:
operation.restore must emit replay-stable evidence for verifier review.
