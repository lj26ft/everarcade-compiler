# World Release Attestation V0.1 RC2

`WORLD_RELEASE_ATTESTATION_V0.1_RC2` is not frozen until independent review passes. The promoted RC2 review bundle was regenerated before freeze verification because the prior exported runtime/deployment artifacts were stale and used the older `world.tick` replay evidence shape; the current verifier deterministically replays `inventory.transfer`, `market.trade`, and `governance.vote` entries from the bundled runtime journal.

## V0.1 RC1 Result

RC1 added trusted-key-gated signature verification, rejected self-signed fake attestations, re-derived `package_hash` from `world.evr`, and re-ran World EVR Package V1 verification.

## Remaining Root-Derivation Gap

Dane's RC1 review found that `world_hash`, `continuity_root`, and runtime verification status claims were still effectively trusted from runtime or deployment reports. A malicious report could say `PASS` while the artifact or runtime evidence did not independently reproduce the same roots.

## RC2 Derivation Recipe

RC2 makes runtime-root verification reproducible from the review artifacts. The current World Factory Phase 2/3 implementation uses canonical JSON with lexicographically sorted object keys and SHA-256 commitments prefixed with `sha256:`.

```text
state_root = sha256(canonical(runtime_state))
receipt_root = sha256(canonical(receipts.map({ tick, receipt_hash, data })))
world_hash = sha256(canonical({ manifest_sha256, contract_hash, runtime_id, world_id }))
```

`manifest_sha256` source and prefix convention:

- Source artifact: the exact bytes of `world.evr/manifest.json` from the World EVR Package V1 directory being attested.
- Digest algorithm: SHA-256 over those bytes, with no newline normalization or JSON reserialization.
- Encoding convention: all hash inputs in this recipe are raw lowercase 64-character hexadecimal strings with no `sha256:` prefix.
- If an upstream manifest or report stores a value with a `sha256:` prefix, verification MUST strip the prefix before canonicalizing inputs; generated V0.1 RC2 attestations MUST emit raw lowercase hex in the recipe fields.

```text
continuity_root = sha256(canonical({ state_root, receipt_root, world_hash, journal }))
```

Replay verification is independently recomputed by bootstrapping genesis state, replaying each journal tick, re-computing the expected receipt hash and journal entry, and comparing the replayed state, receipts, journal, and roots to the supplied runtime artifacts.

Remote verification is reproducible for the exported review bundle by checking that `deploy/deployment-report.json` is for the same world, reports `RUNNING`, and has package, replay, state root, receipt root, world hash, and continuity root fields equal to the independently recomputed values. The report is evidence to compare against; it is not authority for roots or statuses.

## Required Verification Inputs

The review bundle must include:

```text
world.evr/
runtime/world-state.json
runtime/journal.json
runtime/receipts.json
runtime/world-factory-runtime-report.json
deploy/deployment-report.json
release/world-release-attestation.json
release/trusted-public-key.txt
```

## Verification Algorithm

1. Require a trusted public key unless explicitly running a test-only self-attested mode.
2. Compare the trusted key to the attestation embedded public key.
3. Verify the Ed25519 signature over the canonical attestation body without `signature`.
4. Recompute `package_hash` from `world.evr/hash-manifest.json` inputs and compare it to `expected-package-hash.txt` and the attestation claim.
5. Re-run the World EVR Package V1 verifier against `world.evr`.
6. Load runtime state, journal, and receipts from `runtime/`.
7. Replay the journal from the genesis world id and deterministically recompute receipts, runtime state, `state_root`, `receipt_root`, `world_hash`, and `continuity_root`.
8. Compare recomputed roots and statuses to the attestation claims.
9. Compare deployment evidence to recomputed roots/statuses to produce remote verification.
10. Return `PASS` only if every check passes.

## Failure Cases

Verification must fail for missing trusted keys, wrong trusted keys, invalid signatures, package hash mismatch, V1 package verification failure, world hash mismatch, continuity root mismatch, non-reproducible replay verification, non-reproducible remote verification, a status claim of `PASS` when recomputation fails, runtime journal tampering, runtime receipt tampering, runtime state tampering, artifact package mismatch, and artifact root mismatch.

## Non-Goals

RC2 does not implement an operator identity registry, wallet identity, XRPL/Xahau anchoring, legal certification, or key revocation.
