# Frontier Settlement RC1 Operator Guide

This guide is the public operator handoff for Frontier Settlement RC1. Operators do not need compiler internals; they need the generated project, proof bundle, attestation key, and the commands below.

## Workspace layout

- `examples/world-factory/frontier-settlement/world-blueprint.json` — source blueprint.
- `examples/world-factory/frontier-settlement/world-contract-plan.json` — deterministic mutation plan.
- `examples/world-factory/frontier-settlement/out/world.evr/` — generated package.
- `examples/world-factory/frontier-settlement/out/runtime/` — persistent state, journal, receipts, replay report.
- `examples/world-factory/frontier-settlement/out/deploy/` — host bundle and deployment proof.
- `exports/founding-world-rc1-proof-bundle/` — files to publish for third-party verification.

## Operating commands

```bash
node creator-sdk/cli/everarcade.mjs world factory generate
node creator-sdk/cli/everarcade.mjs world factory verify
node creator-sdk/cli/everarcade.mjs world factory boot
node creator-sdk/cli/everarcade.mjs world factory run --ticks 6
node creator-sdk/cli/everarcade.mjs world factory replay
node creator-sdk/cli/everarcade.mjs world factory deploy --host https://founding-world-rc1.everarcade.example
node creator-sdk/cli/everarcade.mjs world attest create --attester-name frontier-settlement-operator-rc1
node creator-sdk/cli/everarcade.mjs world factory serve --host 127.0.0.1 --port 8787
node creator-sdk/cli/everarcade.mjs world factory proof --url http://127.0.0.1:8787
node creator-sdk/cli/everarcade.mjs world attest verify --trusted-public-key "$(cat examples/world-factory/frontier-settlement/out/release/trusted-public-key.txt)"
```

## Verification checklist

A deployment is RC1-valid when all of these are true:

- Package verifier prints `World Factory Verify: PASS`.
- Runtime replay prints `PASS` and `world-factory-runtime-report.json` has `replay_status: PASS`.
- `/health`, `/state`, `/journal`, and `/verify` are reachable on the live host.
- `live-deployment-proof.json` has `deployment_status: RUNNING`, `remote_verification: PASS`, and `attestation_verification: PASS`.
- Release attestation verifies with the trusted public key in the proof bundle.

## Runtime authority

The runtime accepts deterministic World Factory ticks only. Each three-tick cycle applies:

1. `inventory.transfer` — Alice contributes timber to Founders Crossing.
2. `market.trade` — Bruno sells ore to Alice for coin.
3. `governance.vote` — Alice votes yes on opening the public workshop.

The host may change endpoint addresses and process supervision, but must not edit `world.evr`, runtime journal, receipts, or attestation files outside a new signed release.
