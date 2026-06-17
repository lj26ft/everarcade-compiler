# Arena Vanguard Environment Dependency Audit

## Consensus Path

No `process.env`, `process.cwd()`, or `process.argv` values are used by `applyArenaInput`, `executeInput`, `commitFor`, `canonicalize`, `canonicalHash`, or `replayJournal`. These functions derive commitments from validated input envelopes, prior state, receipts, and sorted canonical serialization only.

## Runtime Configuration Path

- `hotpocket-arena-wrapper/src/runtime.mjs`: `defaultPaths(root)` requires callers to provide an explicit runtime root for persistence file locations. Paths are not hashed into state, receipts, journal hashes, world hashes, or continuity roots.
- `hotpocket-arena-wrapper/contract/index.mjs`: `EVERARCADE_REPO_ROOT`, `process.cwd()`, and `PORT` configure storage root and HTTP listener. They remain outside consensus commitments.
- `hotpocket-arena-wrapper/contract/index.mjs`: `process.argv[1]` is used only for direct-execution detection.
- `hotpocket-arena-wrapper/validation/verify-live-path.mjs`: `EVERARCADE_REPO_ROOT` and `process.cwd()` select report and test artifact paths for validation tooling only.
- `hotpocket-arena-wrapper/bin/arena-submit.mjs`: `process.argv` parses CLI input, and `ARENA_HOTPOCKET_URL` chooses a live wrapper URL. This client command is not part of consensus execution.

## Classification

All current environment dependencies are runtime configuration or tooling paths. None are consensus-path dependencies.
