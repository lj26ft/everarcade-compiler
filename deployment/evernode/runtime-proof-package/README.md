# Evernode Lease Execution Proof Package v0.1

This package is the real-lease certification harness for the EverArcade runtime proof. It is intentionally gated so it cannot emit the final certification `PASS` line from a mock lease, a localhost target, or a Docker-only local run.

## Required operator environment

Set these variables on the deployment workstation before running `scripts/prove-lease-execution.sh`:

- `EVERNODE_LEASE_ID`: real Evernode lease identifier.
- `EVERNODE_HOST_ID`: Evernode host identifier for the lease.
- `EVERNODE_SSH_TARGET`: SSH target for the real lease, for example `evernode-user@example.testnet-host`.
- `XRPL_TESTNET_SEED`: funded XRPL Testnet seed used only for the proof transaction.
- `XRPL_TESTNET_DESTINATION`: funded XRPL Testnet destination account for the one-drop memo payment.

Optional variables:

- `XRPL_TESTNET_ENDPOINT` defaults to `wss://s.altnet.rippletest.net:51233`.
- `EVERNODE_REMOTE_ROOT` defaults to `~/everarcade-runtime-proof-package`.
- `EVERARCADE_PREVIOUS_ANCHOR_HASH` defaults to `genesis`.

## Execution

```bash
cd deployment/evernode/runtime-proof-package
scripts/prove-lease-execution.sh
```

The script performs lease discovery, package transfer, runtime startup, deterministic gameplay execution, receipt/journal/checkpoint/state-root generation, continuity anchor generation, real XRPL Testnet publication, retrieval verification, and artifact-only replay validation.

## Certification rule

`Evernode Lease Execution Proof v0.1: PASS` is written only when every required report has `"status": "PASS"`, including the XRPL Testnet report with `tesSUCCESS` and matching retrieved memo hashes.
