# Evernode Live Gameplay Deployment Proof v0.1

This document describes the first IRL-style deployment proof for hosting a playable EverArcade Arena session in an Evernode-style lease environment.

**This proves live gameplay deployment on an Evernode-style environment.**

It does not prove production hosting, public testnet readiness, paid settlement, Unity/Godot support, or general game engine compatibility.

## Deployment package

The lease package is produced at:

```text
dist/everarcade-live-gameplay-proof.tar.gz
```

The package contains:

- `everarcade-runtime/` with the runtime crate source currently present in the repository.
- `creator-sdk/` with Creator SDK package assets.
- `arena-runtime-package/arena_live_runtime.mjs`, the minimal Arena runtime used by this proof.
- `operator-scripts/` for environment probing and Arena session startup.
- `health-scripts/` for HotPocket boundary and frontend access checks.
- `recovery-scripts/` for restart/recovery and network failure probes.
- `frontend/arena-live-client/` with the static browser client.

Build it with:

```bash
bash scripts/build_live_gameplay_proof_package.sh
```

## Lease assumptions

The proof targets a Linux lease-style VM/container with:

- Bash, Node.js, curl, tar, gzip, and Python 3 available.
- A writable working directory for checkpoints, receipts, replay evidence, and reports.
- A bindable TCP port for the runtime API and a bindable TCP port for the frontend.
- Network/firewall policy that can expose the frontend port when a real public lease requires remote access.

`scripts/probe_evernode_environment.sh` records hostname, kernel, disk, memory, CPU, ports, interfaces, process limits, filesystem write permissions, and tool availability in `reports/evernode_environment_probe_report.txt`.

## HotPocket boundary

`scripts/validate_hotpocket_runtime_boundary.sh` checks only the runtime boundary expected of a HotPocket-style host:

- Runtime process can start.
- Inputs can be submitted.
- Outputs can be exported through the state endpoint.
- Checkpoints can be written.
- Replay evidence can be exported.

No consensus, validator agreement, settlement, or production HotPocket claims are made by this proof.

## Runtime process lifecycle

The proof runtime is `scripts/lib/arena_live_runtime.mjs`. It starts an HTTP process, writes status and logs, accepts JSON actions, updates deterministic Arena state, and persists artifacts under `.everarcade-live-proof/` unless a script overrides `EVERARCADE_LIVE_DATA_DIR`.

Important endpoints:

- `GET /health` returns process status, PID, tick, and state root.
- `GET /state` returns the current Arena session state.
- `POST /action` accepts `join`, `move`, `attack`, and `score` actions.
- `POST /checkpoint` writes a checkpoint.
- `GET /replay` exports replay proof metadata.

The live session script starts the runtime, joins `player-a` and `player-b`, executes move/attack/score actions, writes receipts, writes the journal, writes a checkpoint, exports replay evidence, and expects `Live Arena Session: PASS`.

## Frontend access

The minimal frontend lives in `frontend/arena-live-client/` and provides:

- Join.
- Move North.
- Attack.
- Refresh State.
- Player list.
- Health.
- Score.
- Tick.
- State root.
- Replay status.

`scripts/validate_frontend_access.sh` starts the runtime and static frontend server bound to `0.0.0.0`, verifies the page can be fetched, verifies the runtime status endpoint, fetches state, and submits an action. In local CI/container runs this validates the local and interface route; true Internet reachability still depends on the Evernode lease firewall/NAT configuration.

## Resource limits and measurements

`scripts/monitor_live_session_resources.sh` records:

- Disk usage before and after.
- Memory availability before and after.
- CPU sample for the runtime process.
- Runtime process ID and RSS.
- Runtime log size.
- Receipt count.
- Journal size.
- Checkpoint size.

The output report is `reports/live_session_resource_report.txt`.

## Recovery behavior

`scripts/validate_live_session_recovery.sh` starts a session, executes actions, writes a checkpoint, stops the runtime, restarts it against the same data directory, and verifies that:

- The state root before stop matches the state root after restart.
- The replay proof final root matches the restored state root.

The output report is `reports/live_session_recovery_report.txt`, and the expected result is `Live Session Recovery: PASS`.

## Failure/recovery behavior

`scripts/probe_network_failure_behavior.sh` simulates:

- A delayed client request.
- A duplicate action.
- An out-of-order sequence.
- An invalid action.
- A dropped action represented by a sequence gap.

The runtime rejects invalid and out-of-order inputs with deterministic state preservation and accepts the later gap-resolving action.

## Certification

Run the full certification with:

```bash
bash scripts/certify_evernode_live_gameplay.sh
```

Required PASS sections:

```text
Environment Probe: PASS
Package Layout: PASS
HotPocket Boundary: PASS
Runtime Start: PASS
Arena Session: PASS
Frontend Access: PASS
Resource Monitoring: PASS
Recovery: PASS
Network Failure Probe: PASS
Replay Verification: PASS
Evernode Live Gameplay Deployment Proof v0.1: PASS
```

## Known limitations

- This is not a production hosting launch.
- This is not public testnet readiness.
- This does not prove paid settlement or XRPL/Evernode billing behavior.
- This does not prove Unity full game support.
- This does not prove Godot full game support.
- This does not prove Doom/Diablo-style full game support.
- This does not prove general engine upload compatibility.
- The currently supported game class is the Arena template, Civilization template proof, EverArcade WASM guest packages, and Creator SDK packaged games.
