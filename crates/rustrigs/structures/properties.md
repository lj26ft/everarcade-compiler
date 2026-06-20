# Property Targets: structure.build


## Property: Declared Mutation Safety

Given:
Valid structure.build pre-state satisfying declared state domains.

When:
`structure.build()` executes under declared mutation authority.

Then:
structure.build must reject invalid preconditions before mutating canonical state.


## Property: State Integrity

Given:
Valid structure.build pre-state satisfying declared state domains.

When:
`structure.build()` executes under declared mutation authority.

Then:
structure.build must preserve unrelated state domains.


## Property: Continuity Trace

Given:
Valid structure.build pre-state satisfying declared state domains.

When:
`structure.build()` executes under declared mutation authority.

Then:
structure.build must emit replay-stable evidence for verifier review.

# Property Targets: structure.repair


## Property: Declared Mutation Safety

Given:
Valid structure.repair pre-state satisfying declared state domains.

When:
`structure.repair()` executes under declared mutation authority.

Then:
structure.repair must reject invalid preconditions before mutating canonical state.


## Property: State Integrity

Given:
Valid structure.repair pre-state satisfying declared state domains.

When:
`structure.repair()` executes under declared mutation authority.

Then:
structure.repair must preserve unrelated state domains.


## Property: Continuity Trace

Given:
Valid structure.repair pre-state satisfying declared state domains.

When:
`structure.repair()` executes under declared mutation authority.

Then:
structure.repair must emit replay-stable evidence for verifier review.

# Property Targets: structure.decay


## Property: Declared Mutation Safety

Given:
Valid structure.decay pre-state satisfying declared state domains.

When:
`structure.decay()` executes under declared mutation authority.

Then:
structure.decay must reject invalid preconditions before mutating canonical state.


## Property: State Integrity

Given:
Valid structure.decay pre-state satisfying declared state domains.

When:
`structure.decay()` executes under declared mutation authority.

Then:
structure.decay must preserve unrelated state domains.


## Property: Continuity Trace

Given:
Valid structure.decay pre-state satisfying declared state domains.

When:
`structure.decay()` executes under declared mutation authority.

Then:
structure.decay must emit replay-stable evidence for verifier review.
