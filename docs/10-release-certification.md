# 10. Release Certification

This document describes release policy. Implementation scripts and reports are supporting evidence, not the policy source.

## Release Gates

| Gate | Required Evidence |
|---|---|
| Build reproducibility | Offline or pinned dependency build evidence and artifact hashes. |
| Package validation | Package manifest, compatibility, signature, and hash verification. |
| Deterministic execution | Targeted deterministic execution tests and replay roots. |
| State and receipt integrity | State root, receipt hash, journal, and checkpoint verification. |
| Recovery | Backup, restore, checkpoint, and replay recovery drill. |
| Upgrade | Pre-upgrade checkpoint, migration evidence, rollback plan, post-upgrade verification. |
| Federation | Peer exchange, divergence detection, rejoin, and partition recovery tests when federation is in scope. |
| Deployment | Install, start, stop, restart, health, rollback, and artifact verification on target provider. |
| Operations | Runbook coverage, incident response, metrics, and operator handoff. |
| Security | Threat model review, capability boundary review, and external audit scope for production releases. |

## Certification Process

1. define release scope;
2. freeze package and runtime artifact inputs;
3. run required validation gates;
4. collect hashes, manifests, logs, and reports;
5. review failures and waivers;
6. approve or reject release;
7. archive certification evidence.

## Validation Process

Validation must be targeted to the release scope. A release that changes only documentation does not need full runtime certification. A release that changes execution, state, receipt, checkpoint, recovery, federation, or deployment behavior must run the affected gates.

## Offline Build Requirements

Production releases require reproducible build instructions, pinned dependency sources, recorded toolchain versions, checksums for distributed artifacts, and evidence that a clean environment can rebuild or verify the release.

## Artifact Verification

Every release artifact must have a manifest entry, checksum, version, source reference, and expected deployment location. Operators must verify artifacts before activation.

## Recovery Validation

Recovery validation must prove that an operator can restore a world from checkpoint and replay material, detect corrupted artifacts, and resume from verified state.

## Upgrade Validation

Upgrade validation must prove that the old runtime can stop safely, a pre-upgrade checkpoint exists, the new runtime can verify continuity, and rollback remains possible until post-upgrade gates pass.

## Evernode Certification

Evernode certification additionally requires provider install validation, host configuration review, capacity baseline, storage retention policy, backup location, network assumptions, and operator access controls.

## Release Approval Process

A release is approved only when all required gates pass or documented waivers are accepted by the release owner. Waivers must include risk, scope, expiration, and mitigation.
