# Live Evernode Lease Deployment v0.1

## Purpose

Live Evernode Lease Deployment v0.1 validates that the EverArcade Lease Handoff Package can be installed into a lease-style filesystem, operated from that lease root, and certified without changing protocol behavior.

The lease is treated as infrastructure. EverArcade owns protocol state, replay evidence, continuity evidence, settlement evidence, and authority evidence.

## Deployment Workflow

Run:

```bash
bash scripts/deploy_to_evernode_lease.sh
```

The deployment script:

1. Locates the `everarcade-lease-handoff-v0.1.tar.gz` package.
2. Builds the package first when the default package is absent and automatic build is enabled.
3. Verifies the package checksum with the companion `.sha256` file.
4. Extracts the handoff package into the lease handoff area.
5. Verifies `manifests/deployment-manifest.txt`.
6. Initializes the lease layout with runtime, node, HotPocket, Evernode, script, documentation, manifest, checksum, report, and metadata directories.
7. Writes `reports/live_lease_deployment_report.txt`.
8. Updates `reports/live_lease_failures.txt` with exact failures or `No Deployment Failures Observed`.

The default lease root is:

```text
tmp/live-evernode-lease
```

Operators may override it with:

```bash
EVERARCADE_LIVE_LEASE_ROOT=/path/to/lease bash scripts/deploy_to_evernode_lease.sh
```

## Validation Workflow

Run:

```bash
bash scripts/validate_live_lease.sh
```

The validation script checks:

- Runtime assets.
- Protocol node assets.
- HotPocket assets.
- Evernode assets.
- Operator assets.

It then runs the lease-local node lifecycle scripts from the initialized lease root and writes `reports/live_lease_validation_report.txt`.

## Runtime Lifecycle

The lifecycle validation performs:

1. Node initialization.
2. Runtime start through the protocol node lifecycle.
3. Health check through node status.
4. Checkpoint creation.
5. Replay verification.
6. Restore verification.
7. Runtime stop.

The validation report captures:

- Checkpoint identifier.
- Replay root.
- Live continuity root.
- Node restore continuity root.

The pass condition for this milestone is that the restored live continuity root used by the handoff validation matches the replay root. Existing node restore reports are preserved separately for diagnostic comparison.

## Environment Observations

Validation records a lease environment summary containing:

- Disk space.
- Memory availability.
- CPU count.
- Filesystem layout.
- Runtime constraints.

These values are observations only. No optimization, tuning, or resource policy is introduced in this milestone.

## Failure Registry

Failures are written to:

```text
reports/live_lease_failures.txt
```

Each failure records:

- Failure identifier.
- Component.
- Error.
- Impact.
- Suggested fix.

If no deployment or validation failures occur, the registry contains:

```text
No Deployment Failures Observed
```

## PASS Criteria

Certification passes only when all of the following are true:

```text
Deployment: PASS
Validation: PASS
Runtime Start: PASS
Health: PASS
Checkpoint: PASS
Replay: PASS
Restore: PASS
Stop: PASS
Replay Continuity Root Match: PASS
Live Evernode Lease Deployment: PASS
```

## FAIL Criteria

Certification fails when any required package, checksum, manifest, asset class, lifecycle step, root comparison, metadata write, or report generation step fails.

Failures must remain visible in reports. Scripts must not suppress failures or silently convert a failed deployment into a pass.

## Relationship To Lease Handoff Package

The live lease deployment consumes the Lease Handoff Package as the canonical input artifact. The package remains the boundary between build-time preparation and lease-time operation.

The deployment script initializes a lease filesystem from the package but does not alter replay, continuity, settlement, or authority roots.

## Relationship To Future XRPL Settlement Layer

This milestone intentionally excludes XRPL RPC, Hooks, Xaman signing, public settlement, and federation networking.

A passing live lease certification becomes the prerequisite for XRPL Live Settlement Layer v0.1 because it proves the runtime stack can be deployed, started, health checked, checkpointed, replayed, restored, and stopped inside a lease-style environment before settlement authority is introduced.
