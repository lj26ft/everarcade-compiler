# Open Source Candidate RC2

Open Source Candidate RC2 closes the hostile-contributor findings from RC1 by moving reviewer trust out of generated artifacts and making payload binding, gate proofs, and negative fixtures explicit.

## RC2 Deliverables

- `TRUST_ROOT.md` defines the official review key, fingerprint, trust-root version, pinned commit SHA, rotation procedure, and revocation procedure.
- `PAYLOAD_BINDING.md` documents the transitive hash chain from `world.evr` through manifests to blueprint, contract-plan, runtime, replay/journal, and deployment payloads.
- `REVIEW_TRUST_CHAIN.md` documents exactly what the reviewer trusts and where trust begins.
- `GATE_PROOFS.md` documents what Contributor, World Artifact, and Attestation gates prove and do not prove.
- `OPEN_SOURCE_CANDIDATE_RC2_REVIEW_BUNDLE/` contains reviewer workflow, checklist, expected outputs, must-pass reference world description, and must-fail fixture descriptions.

## Critical RC1 Fixes

1. Reviewers obtain the trusted public key from `TRUST_ROOT.md`, never from generated release output.
2. The World Artifact Gate verifies attestations with the committed trust root.
3. Payload tampering is specified as a must-fail condition through explicit manifest and hash-manifest binding.
4. GitHub Actions references used by the candidate workflows are pinned to commit SHAs.
5. Determinism evidence requires two isolated generations and output comparisons.

## Reviewer Success Questions

An independent reviewer can answer without source-code inspection:

- The trust root is `TRUST_ROOT.md` version `everarcade-review-root-v1`.
- Trust begins at the reviewer-selected trust-root document, not generated output.
- Hashes bind `world.evr`, `manifest.json`, `hash-manifest.json`, and every payload digest.
- The attestation signs claims over the package, manifest, replay, and root-equivalence evidence.
- Gate proof boundaries are documented in `GATE_PROOFS.md`.
- Self-attested forks and tampered payloads must fail.
- Tampering is detected by package digest verification and attestation root/signature verification.
