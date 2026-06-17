# Arena Vanguard Projection Demo

This demo presents the Arena Vanguard runtime as a public-facing live projection:

Player Input → Live Runtime → World State Update → Projection Dashboard → Replay Verification

## Dashboard

Start the wrapper from the repository root and open the projection route served by the same process:

```bash
EVERARCADE_REPO_ROOT=$(pwd) PORT=8787 node hotpocket-arena-wrapper/contract/index.mjs
open http://127.0.0.1:8787/projection/dashboard.html
```

For the public VPS deployment, use the same path on the live host:

```text
http://87.99.158.181:8787/projection/dashboard.html
```

The dashboard is non-authoritative. It polls `/state` for the live world and `/verify` for replay status and commitment roots.

## Clean Reset Procedure

For a repeatable recording, stop the wrapper, remove the persisted wrapper state, and restart it with an explicit runtime root:

```bash
rm -f evernode/hotpocket/arena-wrapper-state.json evernode/journals/arena-hotpocket-journal.json
EVERARCADE_REPO_ROOT=$(pwd) PORT=8787 node hotpocket-arena-wrapper/contract/index.mjs
```

The runtime core requires this root to be supplied by the wrapper, CLI, or validation tooling. The root is runtime configuration for persistence paths only; it is not consensus state and is not included in state roots, receipt roots, world hashes, or continuity roots.

## Recording Flow

Submit the demo inputs against the wrapper while the browser is open:

```bash
ARENA_HOTPOCKET_URL=http://87.99.158.181:8787 node hotpocket-arena-wrapper/bin/arena-submit.mjs join --player player-1
ARENA_HOTPOCKET_URL=http://87.99.158.181:8787 node hotpocket-arena-wrapper/bin/arena-submit.mjs join --player player-2
ARENA_HOTPOCKET_URL=http://87.99.158.181:8787 node hotpocket-arena-wrapper/bin/arena-submit.mjs move --player player-1 --direction east
ARENA_HOTPOCKET_URL=http://87.99.158.181:8787 node hotpocket-arena-wrapper/bin/arena-submit.mjs move --player player-2 --direction west
ARENA_HOTPOCKET_URL=http://87.99.158.181:8787 node hotpocket-arena-wrapper/bin/arena-submit.mjs attack --player player-1 --target player-2
curl http://87.99.158.181:8787/verify
```

Expected final visual state:

- `player-1` at `(1, 0)`, health `100`, score `10`.
- `player-2` at `(-1, 0)`, health `75`, score `0`.
- Combat log: `player-1 attacked player-2 for 25 damage`.
- Replay badge: `VERIFIED`.
- State root, receipt root, world hash, continuity root, recent journal entries, and commitment history visible.

## Closing Card

```text
Arena Vanguard
Live deterministic world projection
Replay verified
Build Worlds. Not Just Games.
```
