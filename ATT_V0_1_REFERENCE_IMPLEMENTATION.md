# World Release Attestation V0.1 Reference Implementation Lock

## Locked Protocol Artifact

- **Frozen protocol:** `WORLD_RELEASE_ATTESTATION_V0_1`
- **Review candidate promoted:** `WORLD_RELEASE_ATTESTATION_V0_1_RC2`
- **Spec source at freeze:** `specs/world-attestation/WORLD_RELEASE_ATTESTATION_V0_1_RC2.md`
- **Spec SHA-256 at freeze:** `7f93a64c3ae8a2ec1c2e38892cfa980afbec3a1209232bbe6606feda81a8fa1a`

## Reference Implementation

- **Reference verifier:** `creator-sdk/cli/everarcade.mjs` (`world attest verify`)
- **Reference verifier SHA-256 at freeze:** `c40eb714874aaf2084dbbd35ec1b28b2988a7b0cf1c48c3c5118ea481b9a13fd`
- **Reference commit SHA:** `6e809490cf5ebfbab0d0fa7d78bf61f88ce96fbc`
- **Independent verifier SHA:** `c40eb714874aaf2084dbbd35ec1b28b2988a7b0cf1c48c3c5118ea481b9a13fd`
- **Review bundle SHA-256:** `b1b19498580c94d36d352ceb717a0e8a99fc9e1db8a5810252dd53bf612effc7`

The independent verifier SHA identifies the verifier artifact used to reproduce the RC2 review checks. The review bundle SHA is the aggregate SHA-256 over the tracked files in `exports/world-release-attestation-v0-1-rc2-review/` plus the verifier/spec lock inputs used for this freeze record.

## Locked Verification Requirements

A V0.1 verifier must:

1. parse the attestation as UTF-8 JSON;
2. require an out-of-band trusted public key unless explicitly running a test-only self-attested mode;
3. compare the trusted public key to `attester.public_key`;
4. canonicalize the attestation without `signature` using lexicographically sorted object keys, source-order arrays, JSON scalar encoding, and no insignificant whitespace;
5. verify the Ed25519 signature over the canonical attestation body;
6. recompute `package_hash` from the supplied `world.evr` package and compare it to the attestation claim;
7. run World EVR Package V1 verification;
8. replay runtime evidence from genesis and recompute state, receipts, journal, `state_root`, `receipt_root`, `world_hash`, and `continuity_root`;
9. compare deployment evidence to recomputed package, replay, state, receipt, world, and continuity values;
10. return `PASS` only when every signed `PASS` claim is reproduced by verifier-side evidence.

## Freeze Test Record

The freeze acceptance suite consists of:

- valid RC2 attestation verification with the pinned trusted public key;
- all RC2 failure fixtures rejecting;
- explicit missing-trusted-key rejection;
- runtime root re-derivation via journal, receipt, and state tamper fixtures;
- independent verifier reproduction of valid and negative results.

The frozen review outcome for that suite is `PASS`.

## Version Governance

The V0.1 line is closed. Any breaking change to schema version, canonicalization, signature input, trusted-key requirements, `world_hash` derivation, `continuity_root` derivation, or verification requirements must be released as `WORLD_RELEASE_ATTESTATION_V0_2` or later.
