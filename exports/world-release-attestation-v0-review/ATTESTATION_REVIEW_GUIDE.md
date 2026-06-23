# World Release Attestation V0 Review Guide

This bundle is self-contained for reviewing `WORLD_RELEASE_ATTESTATION_V0` without portal knowledge.

## Bundle Contents

- `WORLD_RELEASE_ATTESTATION_V0.md` — specification under review.
- `world-release-attestation.json` — example signed attestation.
- `release-report.json` — example release report containing `attestation_hash`.
- `world.evr/` — example world package bound by `package_hash`.
- `THREAT_MODEL.md` — assumptions, non-goals, and review boundary.
- `REVIEW_REQUEST.md` — adversarial targets requested from reviewers.
- `failure-fixtures/` — tampered attestations that must fail verification.
- `ATT_V0_VERDICT_TEMPLATE.md` — template for reviewer verdicts.

## Create

From the repository root, regenerate the reference release if needed:

```bash
node creator-sdk/cli/everarcade.mjs world factory boot --project examples/world-factory/frontier-settlement
node creator-sdk/cli/everarcade.mjs world factory replay --project examples/world-factory/frontier-settlement
node creator-sdk/cli/everarcade.mjs world factory deploy --project examples/world-factory/frontier-settlement
node creator-sdk/cli/everarcade.mjs world attest create --project examples/world-factory/frontier-settlement
```

Expected outcome: the attestation and release report are written under `examples/world-factory/frontier-settlement/out/release/` and the create command reports `World Attest Create: PASS`.

## Verify

Verify the bundled positive attestation from the repository root:

```bash
node creator-sdk/cli/everarcade.mjs world attest verify \
  --project examples/world-factory/frontier-settlement \
  --attestation exports/world-release-attestation-v0-review/world-release-attestation.json \
  --world exports/world-release-attestation-v0-review/world.evr
```

Expected outcome:

```text
PASS
```

## Tamper Test

Each fixture in `failure-fixtures/` mutates one signed or verified value while preserving the original signature. Run each fixture with the same world package:

```bash
for fixture in exports/world-release-attestation-v0-review/failure-fixtures/*; do
  printf '%s: ' "$fixture"
  node creator-sdk/cli/everarcade.mjs world attest verify \
    --project examples/world-factory/frontier-settlement \
    --attestation "$fixture/world-release-attestation.json" \
    --world exports/world-release-attestation-v0-review/world.evr || true
done
```

Expected outcome: every fixture prints `FAIL` or otherwise exits non-zero because the tampered attestation is not acceptable.

## Expected Outcomes

| Case | Expected Result | Reason |
| --- | --- | --- |
| Reference attestation | `PASS` | Signature, statuses, package hash, and bundled world package agree. |
| `modified-timestamp` | `FAIL` | Timestamp is signed and mutation invalidates the signature. |
| `modified-package-hash` | `FAIL` | Package hash is signed and must match `world.evr`. |
| `modified-world-hash` | `FAIL` | World hash is signed and mutation invalidates the signature. |
| `modified-continuity-root` | `FAIL` | Continuity root is signed and mutation invalidates the signature. |
| `modified-public-key` | `FAIL` | Public key mutation breaks signature verification. |
| `modified-verification-status` | `FAIL` | Verification statuses are signed and must all be `PASS`. |
