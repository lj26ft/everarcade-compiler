# EverArcade Open Source Candidate RC1 Slice

This directory is the self-contained review slice for Frontier Settlement RC1. It points reviewers only at public repository files needed to reproduce the candidate.

## Workflow

```bash
git clone <repo>
cd everarcade-compiler
bash scripts/check_prerequisites.sh
bash scripts/ensure_vendor_offline.sh
bash scripts/validate_open_source_readiness.sh
EVERARCADE_DETERMINISTIC_ATTEST=1 bash scripts/ci/run-deterministic-world-factory.sh
node creator-sdk/cli/everarcade.mjs world attest verify \
  --project examples/world-factory/frontier-settlement \
  --trusted-public-key "$(awk '/^```text$/{block++; next} block==1 && /^[A-Za-z0-9+\/=]+$/{print; exit}' TRUST_ROOT.md)"
```

Expected terminal results: `Prerequisites: PASS`, `READY`, `Deterministic World Factory CI: PASS`, and attestation verification `PASS`.

## Contents

- `reference-world/` — Frontier Settlement RC1 source blueprint and contract plan.
- `workflows/` — contributor and world artifact CI workflow definitions.
- `gate-scripts/` — prerequisite gate script used before execution.
- `expected-outputs/` — PASS criteria for reviewer comparison.
