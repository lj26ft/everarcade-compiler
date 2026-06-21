# Live Input Walkthrough

## Start the wrapper

```bash
PORT=8787 node hotpocket-arena-wrapper/contract/index.mjs
```

## Submit a demonstration sequence

```bash
node hotpocket-arena-wrapper/bin/arena-submit.mjs join --player player-1
node hotpocket-arena-wrapper/bin/arena-submit.mjs join --player player-2
node hotpocket-arena-wrapper/bin/arena-submit.mjs move --player player-1 --direction north
node hotpocket-arena-wrapper/bin/arena-submit.mjs attack --player player-1 --target player-2
curl http://127.0.0.1:8787/verify
```

## Browser projection

Open `frontend/arena-live-client/index.html` and keep the runtime URL pointed at the wrapper (`http://127.0.0.1:8787`). The projection reads `/state` and submits canonical actions through `/input`; it displays player positions, health, combat events, tick progression, and the latest state root. The browser remains read-only for authority: all state mutations are returned by the wrapper.

## Demo recording checklist

A 60–120 second recording should show the contract starting, a player joining, movement, attack damage, proof/report generation, and replay verification returning `PASS`.
