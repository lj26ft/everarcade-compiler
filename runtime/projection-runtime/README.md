# Projection Runtime v0.1

The Projection Runtime is a downstream visual observer for deterministic EverArcade `ArenaState` snapshots. It is not authoritative, does not join consensus, and exposes no write path to runtime authority, journals, or roots.

## Modes

- **Live mode** projects the latest `ArenaState` frames as they arrive.
- **Replay mode** loads a journal and projects each historical tick.
- **Restore demonstration** reprojects the restored checkpoint continuation tick and verifies visual continuity.
- **Migration demonstration** projects the same migrated `ArenaState` on source and destination runtimes and verifies identical visual output.
- **Operator demonstration** projects Operator A, Operator B, and Operator C from the same state root.

## Commands

```sh
node runtime/projection-runtime/src/cli.mjs --mode live --journal examples/projection-demo-world/journal.json
node runtime/projection-runtime/src/cli.mjs --mode replay --journal examples/projection-demo-world/journal.json
node runtime/projection-runtime/src/certify.mjs
```

## Browser canvas

Serve the repository with any static file server and open:

```text
runtime/projection-runtime/public/index.html?mode=live&operator=Operator%20A
runtime/projection-runtime/public/index.html?mode=replay&operator=Operator%20B
```

The canvas draws a 2D top-down view and the side panel renders players, health, item counts, receipts, continuity, roots, market activity, and governance status exclusively from `ArenaState`.
