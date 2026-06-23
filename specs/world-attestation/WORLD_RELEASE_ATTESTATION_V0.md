# World Release Attestation V0

`WORLD_RELEASE_ATTESTATION_V0` is a portable signed release statement for an EverArcade world artifact. It binds the world artifact hash, verification results, attester key material, and timestamp into an independently verifiable Ed25519 signature.

## Canonical Format

The attestation is UTF-8 JSON with this shape:

```json
{
  "schema_version": "WORLD_RELEASE_ATTESTATION_V0",
  "world_id": "...",
  "package_hash": "...",
  "package_verification": "PASS",
  "replay_verification": "PASS",
  "remote_verification": "PASS",
  "world_hash": "...",
  "continuity_root": "...",
  "timestamp": "...",
  "attester": {
    "name": "...",
    "public_key": "..."
  },
  "signature": "..."
}
```

The `attester.public_key` field is the base64 DER SubjectPublicKeyInfo encoding of an Ed25519 public key. The `signature` field is base64 Ed25519 signature bytes.

## Canonical Manifest and Hash Recipe

The signature does not cover the raw JSON file bytes. It covers `canonical(attestation_without_signature)`.

Canonicalization rules:

1. Remove the top-level `signature` field.
2. Encode JSON recursively with object keys sorted lexicographically.
3. Emit arrays in their existing order.
4. Emit strings, numbers, booleans, and null using standard JSON encoding.
5. Do not add insignificant whitespace.
6. Encode the result as UTF-8.

The attestation hash is:

```text
SHA-256(canonical(attestation_without_signature))
```

The hex digest is named `attestation_hash` in release reports.

## Signature Algorithm

Only Ed25519 is valid. No alternate algorithms, key types, or signature suites are part of V0.

Signing procedure:

```text
signature = Ed25519.sign(private_key, canonical(attestation_without_signature))
```

Verification procedure:

```text
Ed25519.verify(public_key, canonical(attestation_without_signature), signature)
```

## CLI Procedures

Create:

```bash
everarcade world attest create
```

The create command consumes the generated `world.evr`, package verification result, replay verification result, and remote verification result already produced by World Factory Phase 3. It does not reimplement those verification domains. It writes:

```text
out/release/world.evr
out/release/world-release-attestation.json
out/release/release-report.json
```

The release report has this shape:

```json
{
  "world_id": "...",
  "package_hash": "...",
  "attestation_hash": "...",
  "attestation_status": "PASS"
}
```

Verify:

```bash
everarcade world attest verify
```

The verify command canonicalizes the attestation without `signature`, recomputes the attestation hash, verifies the Ed25519 signature with the supplied public key, checks that all status fields are `PASS`, and checks that the package hash matches the supplied/generated world artifact when present. It prints exactly `PASS` or `FAIL`.

## Proof Boundaries

Allowed claims:

- A release artifact was signed.
- The attestation can be independently verified from the attestation, public key, and artifact.
- Ed25519 signature verification passed.

Not claimed by V0:

- Identity proof.
- On-chain registration.
- Legal certification.
- Wallet ownership.

## Future Hooks

The following are intentionally documented but not implemented in V0:

- Operator Identity Registry
- XRPL Anchoring
- Xahau Anchoring
- Xaman Signing
- Multi-Signature Attestations
- Verifier Attestations
- Certification Authorities
