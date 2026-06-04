# Evernode-Compatible Deployment Layer v0.1

## Purpose

Evernode-Compatible Deployment Layer v0.1 adds the first deployment surface beneath the existing EverArcade Protocol Node Appliance and HotPocket Integration Layer. It defines a filesystem layout, bootstrap flow, validation flow, and certification report for future Evernode packaging work.

This phase is layout-only. It does not register with Evernode, upload to a lease, run public networking, execute consensus, call XRPL RPC, integrate Xaman, or invoke Hooks.

## Phase 1 Scope

Phase 1 provides:

- A canonical `evernode/` deployment layout.
- A bootstrap script that creates the layout and writes a deployment manifest.
- A validation script that checks layout, manifest, Protocol Node Appliance scripts, and HotPocket integration scripts.
- A certification script that runs bootstrap, validation, and HotPocket certification.
- Reports under `reports/` for bootstrap, validation, and certification.

## Deployment Layout

```text
evernode/
  lease/        Deployment manifest and future lease metadata.
  runtime/      Future packaged runtime artifacts.
  node/         Future Protocol Node Appliance deployment bindings.
  hotpocket/    Future HotPocket contract/container deployment bindings.
  checkpoints/  Future deployment checkpoint exports.
  journals/     Future deployment journal exports.
  backups/      Future backup artifacts.
  logs/         Future deployment logs.
  reports/      Future deployment-local reports.
```

Empty directories are tracked with `.gitkeep`. Generated runtime artifacts are ignored unless they are Phase 1 source-controlled deliverables.

## Bootstrap Command

```bash
bash scripts/evernode_bootstrap.sh
```

Bootstrap creates the `evernode/` layout, verifies required Protocol Node Appliance and HotPocket integration scripts, writes `evernode/lease/deployment-manifest.txt`, and writes `reports/evernode_bootstrap_report.txt`.

Expected output:

```text
Evernode Bootstrap: PASS
```

## Validation Command

```bash
bash scripts/validate_evernode_deployment.sh
```

Validation verifies:

- Evernode layout exists.
- Deployment manifest exists.
- Protocol node scripts exist.
- HotPocket validation script exists.
- HotPocket certification script exists.

It also runs:

```bash
bash scripts/node_doctor.sh
bash scripts/validate_hotpocket_integration.sh
```

Validation writes `reports/evernode_validation_report.txt`.

Expected output:

```text
Evernode Deployment Validation: PASS
```

## Certification Command

```bash
bash scripts/certify_evernode_deployment.sh
```

Certification runs:

```bash
bash scripts/evernode_bootstrap.sh
bash scripts/validate_evernode_deployment.sh
bash scripts/certify_hotpocket_integration.sh
```

Certification writes `reports/evernode_deployment_certification_report.txt`.

Expected output:

```text
Evernode Deployment Layer: PASS
```

## PASS Criteria

The layer passes when:

- The canonical `evernode/` layout exists.
- The deployment manifest exists.
- Protocol Node Appliance scripts are present.
- HotPocket validation and certification scripts are present.
- Node diagnostics pass through `scripts/node_doctor.sh`.
- HotPocket validation passes through `scripts/validate_hotpocket_integration.sh`.
- Certification passes through `scripts/certify_hotpocket_integration.sh`.
- Reports are generated for bootstrap, validation, and certification.

## FAIL Criteria

The layer fails if:

- Any required Evernode directory is missing.
- The deployment manifest is missing or empty.
- Required Protocol Node Appliance scripts are missing.
- Required HotPocket validation or certification scripts are missing.
- Node diagnostics fail.
- HotPocket validation or certification fails.
- Any live Evernode registration, lease upload, networking, consensus, XRPL RPC, Xaman, or Hooks behavior is introduced in this phase.

## Relationship To HotPocket Integration Layer

The target stack for this phase is:

```text
EverArcade Protocol Node
        ↓
HotPocket Adapter
        ↓
Evernode-Compatible Deployment Layout
```

The Evernode-compatible layout is a deployment boundary below HotPocket. HotPocket remains the adapter layer, and the Protocol Node Appliance remains authoritative for deterministic state, checkpoints, replay, continuity, and settlement roots.

## Future Phases

Future phases may add:

- Package build.
- Install.
- Start/stop.
- Health.
- Recovery.
- Upgrade.
- Live Evernode deployment.

Those phases must preserve the Phase 1 non-goals until explicitly implemented.
