# EverArcade Release Candidate v0.1

## Purpose
EverArcade v0.1 RC determines whether the feature-complete platform can ship. The milestone validates operational readiness, freezes interfaces, certifies upgrade and recovery procedures, and prepares external onboarding for developers, operators, GPU providers, creators, and players.

This release candidate does not add protocol functionality. It certifies the platform that already includes the protocol runtime, civilization runtime, XRPL settlement, Xaman signing, federation runtime, renderer runtime, GPU runtime, GPU marketplace, developer portal, and public testnet.

## Freeze Policy
The v0.1 RC freezes the following interfaces:

- CLI commands and required argument semantics.
- Runtime layout paths for packages, checkpoints, receipts, replay, config, and logs.
- Portal layout sections for onboarding, upload, deployment, GPU marketplace, settlement, replay, and runbooks.
- Deployment package layout for runtime appliance, node appliance, Evernode package, lease handoff package, and portal artifacts.
- Settlement layout for XRPL testnet evidence, Xaman signing, receipt ranges, replay roots, and audit records.
- Federation layout for enrollment, governance, checkpoint sync, receipt propagation, replay activation, and quarantine records.

The interface freeze root is recorded in `release/interface-freeze/INTERFACE_FREEZE_ROOT`.

## Upgrade Policy
Upgrade certification covers install, upgrade, restart, replay, and recovery. Operators must preserve checkpoints, receipts, replay records, and root evidence before upgrading. After restart, operators must replay from the checkpoint boundary and compare continuity roots.

The upgrade root is recorded in `release/upgrade-certification/UPGRADE_ROOT`.

## Recovery Policy
Recovery certification covers checkpoint, restore, replay, and continuity. A recovery is valid only if the restored checkpoint loads, replay records hydrate from the checkpoint boundary, and the resulting continuity evidence matches the recorded lineage.

The recovery root is recorded in `release/recovery-certification/RECOVERY_ROOT`.

## Distribution Model
The release candidate distribution model requires source, artifacts, and the vendor bundle to reproduce the release. Certified distribution surfaces are:

- Runtime Appliance
- Node Appliance
- Evernode Package
- Lease Handoff Package
- Portal Artifacts

The distribution root is recorded in `release/distribution-certification/DISTRIBUTION_ROOT`.

## Testnet Readiness
Public testnet readiness requires enrollment, deployment, governance, GPU marketplace, settlement, and replay to pass. The public testnet is suitable for external onboarding only while all readiness checks remain passing.

The testnet readiness root is recorded in `release/testnet-readiness/TESTNET_READINESS_ROOT`.

## PASS Criteria
EverArcade v0.1 RC passes when:

1. Release manifest root exists and validates.
2. Interface freeze root exists and validates.
3. Upgrade root exists and validates.
4. Recovery root exists and validates.
5. Distribution root exists and validates.
6. Testnet readiness root exists and validates.
7. Deployment, operator, recovery, upgrade, and developer runbooks exist.
8. `bash scripts/validate_release_candidate.sh` reports `Release Candidate Validation: PASS`.
9. `bash scripts/certify_release_candidate.sh` reports `EverArcade Release Candidate v0.1: PASS`.

## FAIL Criteria
EverArcade v0.1 RC fails if any critical workflow cannot be reproduced, any frozen interface is changed without a new freeze root, any upgrade or recovery continuity root mismatches, any required distribution artifact is missing, or public testnet readiness no longer passes.

## Path To Launch
After this release candidate passes, the next milestones are adoption milestones rather than infrastructure milestones:

- Creator SDK v0.1
- Game Templates v0.1
- Creator Marketplace v0.1
- Commercial Launch v0.1
