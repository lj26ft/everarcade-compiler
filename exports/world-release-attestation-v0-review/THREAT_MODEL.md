# World Release Attestation V0 Threat Model

This review bundle captures the current security assumptions, proof boundaries, and explicit non-goals for `WORLD_RELEASE_ATTESTATION_V0` before any freeze decision.

## Security Assumptions

- **Canonical JSON:** The signed payload is the canonical JSON encoding of the attestation with the top-level `signature` field removed. Object keys are sorted lexicographically, arrays remain in-order, and insignificant whitespace is not signed.
- **SHA-256:** Package, world, continuity, and attestation hash commitments use SHA-256-derived digests or commitment strings.
- **Ed25519:** V0 permits only Ed25519 signatures. Alternate signature suites are outside the V0 proof surface.
- **Public Key Verification:** The verifier uses the attestation public key, or an explicitly supplied public key, to verify the signature over the canonicalized payload.
- **Package Hash Binding:** The attestation binds `package_hash` to the reviewed `world.evr` artifact and the release report.
- **Verification Status Binding:** `package_verification`, `replay_verification`, and `remote_verification` are signed and must be `PASS` for acceptance.
- **Timestamp Binding:** The release timestamp is included in the signed payload so timestamp changes invalidate the signature.

## Known Non-goals

- Identity verification of the named attester.
- Wallet ownership proof.
- On-chain anchoring.
- Certificate authority trust chains.
- Multi-signature release authorization.
- Key rotation policy or historical key continuity.

## Review Boundary

A valid V0 attestation should prove that a specific world artifact and verification result set were signed by the private key corresponding to the included or supplied Ed25519 public key. It should not be interpreted as legal certification, operator identity proof, wallet control proof, or chain-anchored finality.
