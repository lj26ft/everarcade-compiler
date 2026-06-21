# Developer Technical Overview

EverArcade developers build deterministic worlds rather than opaque server plugins. A deployable world should define code, assets, initial state, mutation rules, and replay expectations so operators and verifiers can reproduce execution.

## What you build

A serious world package normally includes:

- World Contract code or bindings for accepted mutations.
- Runtime-compatible WASM.
- Manifest metadata and canonical package roots.
- Asset references and initial state material.
- Replay fixtures for local debugging.
- Upgrade notes when package behavior changes.

## World Contracts

World Contracts define mutation rules. They should reject invalid inputs, avoid nondeterministic host behavior, and produce canonical state changes. The contract boundary is **EXPERIMENTAL**, so use it as a developing protocol surface.

## RustRigs

RustRigs is **ALPHA**. It provides reusable gameplay domain libraries and examples, but it is not yet a production standard library. The intended mutation surface looks like:

```rust
combat.attack(attacker, target);
inventory.transfer(item, from, to);
market.trade(order);
quest.complete(player, quest);
```

The important rule is not the exact function names; it is that gameplay mutations should be deterministic, replayable, and rooted in world state.

## Runtime packages

A runtime package turns a built world into something an operator can execute. It binds deterministic WASM and package metadata to a runtime version. Developers should treat runtime version changes as compatibility events and keep replay fixtures current.

## Assets and state packaging

Assets should be content-addressed or otherwise referenced by canonical metadata. Initial state should be encoded deterministically. Large mutable assets should not become hidden authority; state transitions must still be reproducible from package data, inputs, and checkpoints.

## Local replay and debugging

A practical local loop is:

```text
create world -> build WASM -> package -> run inputs -> emit receipts -> replay -> compare roots
```

When debugging, preserve the input that caused the mismatch, the package root, the runtime version, the checkpoint root, and the expected receipt/state roots.

## Deployable package contents

A deployable package should answer four questions:

| Question | Package evidence |
| --- | --- |
| What code runs? | WASM artifact and runtime requirement |
| What state starts? | Genesis state or checkpoint root |
| What mutations are valid? | World Contract / ABI metadata |
| How is it checked? | Replay fixtures, receipts, and expected roots |
