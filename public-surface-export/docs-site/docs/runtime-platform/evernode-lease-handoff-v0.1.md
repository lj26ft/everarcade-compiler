# Evernode Lease Handoff Package v0.1

## Purpose

The Evernode Lease Handoff Package is the canonical deployment artifact that moves EverArcade from a developer machine into a fresh Evernode-compatible lease environment. It proves that a lease operator can receive, verify, install, validate, and prepare the runtime for startup without source repository access.

The package excludes live lease registration, Evernode credentials, XRPL credentials, and Xaman credentials.

## Handoff Layout

The canonical handoff layout is:

```text
handoff/
  package/
  docs/
  scripts/
  manifests/
  checksums/
  reports/
```

The generated tarball preserves the same lease handoff layout under `everarcade-lease-handoff-v0.1/`.

## Package Contents

The package contains:

- Evernode deployment assets from `evernode/`.
- Runtime appliance assets from `runtime/config` and `runtime/deployment`.
- Protocol node assets from `node/` and node operator scripts.
- HotPocket assets from `hotpocket/` and HotPocket validation scripts.
- Operator lifecycle scripts for build, verify, install, validate, and certify.
- Documentation and the operator runbook.
- Deployment manifest and content checksum manifest.
- Reports directory for lifecycle evidence.

## Build Process

Run:

```bash
bash scripts/build_lease_handoff_package.sh
```

The builder stages required assets, writes a deployment manifest, writes per-file checksums, creates `dist/everarcade-lease-handoff-v0.1.tar.gz`, and creates `dist/everarcade-lease-handoff-v0.1.tar.gz.sha256`.

Build output is recorded in `reports/lease_handoff_build_report.txt`.

## Verification Process

Run:

```bash
bash scripts/verify_lease_handoff_package.sh
```

Verification checks the package checksum, extracts the tarball into a temporary verification directory, validates the manifest, validates the required layout, checks required files, and verifies packaged file checksums.

Verification output is recorded in `reports/lease_handoff_verify_report.txt`.

## Installation Process

Run:

```bash
bash scripts/install_lease_handoff_package.sh
```

Installation fails closed. It verifies the package before extraction, removes the previous generated install target, extracts into `handoff/lease-install/`, and confirms the installed manifest and layout.

Installation output is recorded in `reports/lease_handoff_install_report.txt`.

## Validation Process

Run:

```bash
bash scripts/validate_lease_handoff_package.sh
```

Validation checks the installed package for runtime assets, node assets, HotPocket assets, Evernode assets, reports, documentation, manifests, and content checksums.

Validation output is recorded in `reports/lease_handoff_validation_report.txt`.

## Operator Runbook

The concise operator runbook is packaged as `docs/operator-runbook.txt` and tracked in the source layout as `handoff/docs/operator-runbook.txt`.

It covers receiving the package, verifying checksum, installing, validating, starting the runtime, running health checks, recovery, and upgrades.

## PASS Criteria

The lifecycle passes when all commands report:

```text
Lease Handoff Build: PASS
Lease Handoff Verification: PASS
Lease Handoff Installation: PASS
Lease Handoff Validation: PASS
Evernode Lease Handoff Package: PASS
```

The package must have a checksum, deployment manifest, file checksum manifest, required assets, operator scripts, documentation, and installable layout.

## FAIL Criteria

The lifecycle fails if any required asset is missing, the tarball checksum fails, the deployment manifest is missing, the layout is incomplete, packaged file checksums fail, installation cannot verify before extraction, or validation cannot prove the installed runtime, node, HotPocket, Evernode, reports, and documentation assets.

## Relationship To Evernode Deployment Phase 2

Evernode Deployment Phase 2 proved the repository can model an Evernode-compatible deployment. The lease handoff package turns those assets into an immutable operator-facing artifact with independent verification and installation evidence.

## Relationship To Future Live Lease Deployment

This package is the immediate prerequisite for live Evernode lease deployment. Future milestones will add live lease registration, live networking, XRPL settlement, Xaman signing, public testnet deployment, and production runtime activation. Those live capabilities are intentionally excluded from v0.1.
