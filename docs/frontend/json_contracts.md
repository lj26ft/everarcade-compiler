# EverArcade Frontend JSON Contracts

These schemas are frozen for the first creator/player/operator frontend milestone. Frontends consume Product Command Facade JSON through `frontend/shared-api`; they do not call execution-core internals.

## Authority Boundary

Frontend surfaces are view, interaction, command submission, and monitoring only. Runtime authority remains in HotPocket state, replay, checkpoints, receipts, and deterministic runtime execution.

## DoctorResult

```json
{
  "command": "doctor",
  "status": "ready | failed",
  "checks": [{ "name": "Cargo", "status": "passed | warning | failed", "emoji": "✅", "suggested_fix": null }]
}
```

## StatusResult

```json
{
  "command": "status",
  "runtime": "healthy",
  "replay": "healthy",
  "deployment": "ready",
  "federation": "healthy",
  "metrics": { "mode": "scaffold", "deterministic": true }
}
```

Future expanded status providers may replace `runtime`, `replay`, and `federation` string values with `RuntimeStatus`, `ReplayStatus`, and `FederationStatus` objects while preserving the top-level keys.

## PackageResult

```json
{
  "command": "package",
  "status": "complete",
  "runtime_package": "deployment/evernode/runtime/arena-vanguard-runtime.tar.gz",
  "world_package": "deployment/evernode/runtime/arena-vanguard-world.tar.gz",
  "deployment_package": "deployment/evernode/runtime/arena-vanguard-deployment.tar.gz",
  "checksums": "verified"
}
```

## ValidationResult

```json
{
  "command": "validate",
  "profile": "quick",
  "status": "passed",
  "checks": [{ "name": "doctor", "status": "passed", "emoji": "✅", "suggested_fix": null }]
}
```

## DeploymentResult

```json
{
  "command": "deploy",
  "mode": "dry-run",
  "status": "ready",
  "live_evernode": "not-implemented"
}
```

## Provider Compatibility

- CLI JSON provider: shells out to `everarcade <command> --json`.
- HTTP provider: posts through `frontend-gateway` endpoints.
- Websocket provider: reserved for future realtime monitoring and command result streams.
