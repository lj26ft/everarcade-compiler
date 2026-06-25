# EverArcade Trust Root

Trust-root version: `everarcade-review-root-v1`

Pinned repository commit: `b6d553d5632a88b95352d6c98d15503b27d4df0f`

Official EverArcade review key (Ed25519 SubjectPublicKeyInfo, base64):

```text
MCowBQYDK2VwAyEAkRhlmDoAtG0Ib0OAIAVYcj8wqjpwri7ZMbxI0J0DlFo=
```

Fingerprint (SHA-256 over the exact base64 text above, no trailing newline):

```text
1aa333e0a05deda04030f2ebda1c60023160f940208e51688eec2a73dc918de4
```

## Trust Boundary

Reviewers must copy the trusted review key from this committed trust-root document, or from a separately published copy of this exact document. Reviewers must not copy a trusted key from generated build output, release output, a fixture under test, or any artifact whose authenticity is being evaluated.

## Rotation Procedure

1. Create a new trust-root version, for example `everarcade-review-root-v2`.
2. Generate the replacement Ed25519 review key offline.
3. Publish a commit that updates this document with the new public key, fingerprint, trust-root version, activation date, and pinned commit SHA.
4. Sign the rotation announcement with the previous review key when possible and publish it outside the artifact under review.
5. Keep the previous key accepted only for attestations whose signed timestamp predates the activation of the new root.

## Revocation Procedure

1. Publish a revocation notice naming the trust-root version, compromised key fingerprint, affected commit range, and last known-good attestation.
2. Remove the revoked key from reviewer workflows and mark all newer attestations signed by that key as invalid.
3. Publish a replacement trust-root version and pinned commit SHA using the rotation procedure.
4. Require reviewers to re-run attestation verification and must-fail fixtures with the replacement key.

## Reviewer Command

```bash
TRUSTED_PUBLIC_KEY="$(awk '/^```text$/{block++; next} block==1 && /^[A-Za-z0-9+/=]+$/{print; exit}' TRUST_ROOT.md)"
node creator-sdk/cli/everarcade.mjs world attest verify \
  --project examples/world-factory/frontier-settlement \
  --trusted-public-key "$TRUSTED_PUBLIC_KEY"
```
