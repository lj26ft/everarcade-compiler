# EverArcade Public Testnet v0.1 Layout

The `public-testnet/` tree is the deterministic, testnet-only operating ledger for the first EverArcade Public Testnet. It records external-participant workflows without production funds, production revenue, or production asset value.

## Directories

- `developers/` — developer registration, project registration, project approval, and project status records.
- `operators/` — lease operator, node operator, federation operator, health, checkpoint, and replay records.
- `gpu-providers/` — GPU provider registration, capability advertisement, capacity declaration, and provider status records.
- `players/` — player access cohorts for test interactions with deployed games.
- `deployments/` — project deployment, lease assignment, federation membership, and deployment status records.
- `civilizations/` — civilization, world, region, governance state, and economy state records.
- `settlements/` — XRPL testnet settlement intents, Xaman test authorization, observed test settlement, and imported receipt records.
- `gpu-marketplace/` — provider job, artifact, verification, and GPU settlement-intent test records.
- `governance/` — testnet proposal, vote, policy, and rule-change records.
- `analytics/` — derived metrics for developers, deployments, civilizations, GPU jobs, settlement events, and operator activity.
- `replay/` — replay transcript that verifies deterministic reconstruction of the public testnet root.
- `records/` — generated root hashes for each registry and the aggregate public testnet root.
- `reports/` — reserved for testnet-local operator reports; repository-level validation reports are written to `reports/`.

## Determinism

`testnet_model.sh` is the deterministic source of truth. Validation and certification scripts regenerate records, calculate roots, and verify that the replay root equals the public testnet root.
