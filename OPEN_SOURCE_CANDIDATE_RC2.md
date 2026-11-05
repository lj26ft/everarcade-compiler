# Open Source Candidate RC2

Open Source Candidate RC2 closes the hostile-contributor findings from RC1 by moving reviewer trust out of generated artifacts and making payload binding, gate proofs, and negative fixtures explicit.

## RC2 Deliverables

- `TRUST_ROOT.md` records trust-root version `everarcade-review-root-v1`, now REVOKED (`REVOKED_COMPROMISED_TEST_FIXTURE`). Its only key is the committed test fixture, not an official review key. It also records the fingerprint, pinned commit SHA, rotation procedure, and revocation procedure. The replacement is `trust-roots/everarcade-review-root-v2.md` (`PREPARED_NO_ACTIVE_KEY`).
- `PAYLOAD_BINDING.md` documents the transitive hash chain from `world.evr` through manifests to blueprint, contract-plan, runtime, replay/journal, and deployment payloads.
- `REVIEW_TRUST_CHAIN.md` documents exactly what the reviewer trusts and where trust begins.
- `GATE_PROOFS.md` documents what Contributor, World Artifact, and Attestation gates prove and do not prove.
- `OPEN_SOURCE_CANDIDATE_RC2_REVIEW_BUNDLE/` contains reviewer workflow, checklist, expected outputs, must-pass reference world description, and must-fail fixture descriptions.

## Critical RC1 Fixes

1. Reviewers obtain review keys only from an ACTIVE trust-root document, never from generated release output. v1 (`TRUST_ROOT.md`) is REVOKED and v2 (`trust-roots/everarcade-review-root-v2.md`) is `PREPARED_NO_ACTIVE_KEY`, so there is no trusted review key at present.
2. The World Artifact Gate verifies attestations against the committed trust-root document. In RC2 that was the v1 TEST FIXTURE key, now revoked; those results are non-production reproduction only.
3. Payload tampering is specified as a must-fail condition through explicit manifest and hash-manifest binding.
4. GitHub Actions references used by the candidate workflows are pinned to commit SHAs.
5. Determinism evidence requires two isolated generations and output comparisons.

## Reviewer Success Questions

An independent reviewer can answer without source-code inspection:

- The RC2 trust root was `TRUST_ROOT.md` version `everarcade-review-root-v1`, now REVOKED (`REVOKED_COMPROMISED_TEST_FIXTURE`). Its replacement is `trust-roots/everarcade-review-root-v2.md` (`PREPARED_NO_ACTIVE_KEY`, no active key). Reviewers obtain keys only from an ACTIVE trust-root document.
- Trust begins at the reviewer-selected trust-root document, not generated output.
- Hashes bind `world.evr`, `manifest.json`, `hash-manifest.json`, and every payload digest.
- The attestation signs claims over the package, manifest, replay, and root-equivalence evidence.
- Gate proof boundaries are documented in `GATE_PROOFS.md`.
- Self-attested forks and tampered payloads must fail.
- Tampering is detected by package digest verification and attestation root/signature verification.
