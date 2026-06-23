# WORLD_RELEASE_ATTESTATION_V0.1_RC1

V0 is not frozen. `WORLD_RELEASE_ATTESTATION_V0.1_RC1` fixes the V0 self-authenticating key risk: the embedded `attester.public_key` is signed metadata, not a trust root.

## Schema

The attestation is UTF-8 JSON:

```json
{
  "schema_version": "WORLD_RELEASE_ATTESTATION_V0_1_RC1",
  "world_id": "...",
  "package_hash": "...",
  "package_verification": "PASS",
  "replay_verification": "PASS",
  "remote_verification": "PASS",
  "world_hash": "sha256:...",
  "continuity_root": "sha256:...",
  "timestamp": "ISO-8601",
  "attester": { "name": "offline-attester", "public_key": "base64-spki-ed25519" },
  "signature": "base64-ed25519-signature"
}
```

## Canonicalization Rules

The signature covers `canonical(attestation_without_signature)`, not the source file bytes. Objects are sorted lexicographically by key, arrays remain in source order, strings and scalars use JSON encoding, and no insignificant whitespace is included.

## Trusted Key Requirement

Verification MUST receive an out-of-band trusted key via `--trusted-public-key <key>` or `--trusted-key <key>`. The verifier MUST fail when no trusted key is supplied, unless `--allow-self-attested-test-only` is used for explicit local negative testing. The trusted key MUST parse to the same Ed25519 public key as `attester.public_key`; otherwise verification fails with an untrusted attester key result.

## Artifact Re-Derivation Requirement

The verifier MUST re-derive claims from the supplied world artifact and generated reports. It MUST NOT trust signed status strings alone. It checks `package_hash`, package V1 verification, replay verification, remote verification, `world_hash`, and `continuity_root` against the artifact/runtime/deployment reports.

## Verification Procedure

1. Parse attestation JSON.
2. Require an out-of-band trusted public key.
3. Confirm the trusted key matches the embedded public key.
4. Canonicalize the attestation without `signature`.
5. Verify the Ed25519 signature.
6. Recompute package hash from `world.evr` and run the V1 package verifier.
7. Re-read replay and deployment reports and re-derive world/continuity roots from runtime artifacts.
8. PASS only if all signed PASS claims match re-derived PASS evidence.

## Known Non-Goals

V0.1 RC1 does not provide operator identity registry, wallet identity, XRPL/Xahau anchoring, legal certification, or key revocation.

## Future L2/L3 Trust Model

L1 is the pinned trusted key implemented here. Later layers add registries, revocation, continuity, and chain/wallet UX without changing the core requirement that signatures are verified against a trust root outside the attestation.
