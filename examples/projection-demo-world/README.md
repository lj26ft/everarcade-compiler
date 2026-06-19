# Arena Vanguard Projection Demo

This demo world supplies deterministic `ArenaState` snapshots for Projection Runtime v0.1. It shows visible movement, combat health changes, inventory item counts, market trade receipts, governance proposal status, replay, restore/continue, migration, and three operators observing identical state roots.

Run from the repository root:

```sh
node runtime/projection-runtime/src/cli.mjs --mode live --journal examples/projection-demo-world/journal.json
node runtime/projection-runtime/src/cli.mjs --mode replay --journal examples/projection-demo-world/journal.json
node runtime/projection-runtime/src/certify.mjs
```

Open `runtime/projection-runtime/public/index.html` through a static web server to view the canvas projection.
