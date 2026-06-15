# Operators

Operators run worlds, preserve continuity, and make world history independently recoverable.

## What do operators do?

Operators host world packages, execute world updates, retain checkpoints and proof bundles, and coordinate with other infrastructure participants when a world spans multiple nodes.

## How are worlds hosted and verified?

Worlds are packaged with their rules and metadata. Operators run those packages, publish receipts, retain replay material, and support restoration from checkpoints when recovery is needed.

## Replay proofs and federation

Replay proofs let another participant verify a history window. Federation lets operators coordinate world execution, synchronization, and recovery without turning one machine into the permanent source of truth.

## Required links

- [Runtime Operations](/docs/13-runtime-operations-manual)
- [Federation](/docs/architecture/federation/federation-runtime)
- [Proof Bundles](/docs/runtime/replay_verification)
- [Recovery Procedures](/docs/operator-recovery)
