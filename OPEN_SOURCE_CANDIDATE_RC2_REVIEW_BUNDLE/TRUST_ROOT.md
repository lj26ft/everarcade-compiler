> **REVOCATION NOTICE: trust-root `everarcade-review-root-v1` is REVOKED (REVOKED_COMPROMISED_TEST_FIXTURE).**
>
> - **Version revoked:** `everarcade-review-root-v1` (this document).
> - **Reason:** the only key in this trust root is the repository's test attester fixture. Its private key is in the public repository, so anyone can sign with it. It was never a production review key.
> - **Revoked key:** Ed25519 SubjectPublicKeyInfo, base64 `MCowBQYDK2VwAyEADvivFlH4llB2VHSZyKghEuygKr8qzGDnuzLxCLczZkg=`
> - **Revoked key SPKI DER SHA-256:**
>   `6858d411f4f7ef9962de02945a7197ad9994903ec734627701cd1fb79968cd9c`
> - **Affected range:** every attestation that verifies under this key, whatever its timestamp. Attestation timestamps are written by the signer and cannot bound the exposure.
> - **Last known-good attestation:** none established by signature.
> - **Replacement:** `trust-roots/everarcade-review-root-v2.md` (status `PREPARED_NO_ACTIVE_KEY`: it has no active review key yet, so production verification fails closed). Revocations are listed append-only in `trust-roots/REVOCATIONS.md`; existing committed attestations have an explicit status in `trust-roots/ATTESTATION-STATUS-V2.md`.
> - **Rotation Procedure steps 4 and 5 below must not be used** for this revocation: no announcement signed with the revoked key, and no acceptance based on attestation timestamps.
> - **Reviewer Command below:** kept so RC2 history can be reproduced. Its result is **TEST FIXTURE ONLY — not trust**. A `PASS` from it shows only that the fixture key signed; it does not show who signed.
> - **Scope of this notice:** it records a key exposure. It does not claim that the key has been misused; no misuse is known. Known deployed consumers of this trust root: none known. Live use elsewhere: unknown.

# EverArcade Trust Root

Trust-root version: `everarcade-review-root-v1`

RC2 artifact/export commit: `fe51c1ce5be6df888dfaae203d5632580a045f2e`

RC2 reviewer-entry commit: `53a17567e826c5d4f9b083e490cf1568bfe7534e`

> **TEST ONLY - NOT A PRODUCTION TRUST ROOT.** The key below is the committed
> test attester fixture (`fixtures/trust-root/test-attester-public-key.txt`).
> Its private key is committed in this repository and is therefore public.
> Anything it signs is a TEST FIXTURE signature: it must never be trusted for
> production, mainnet, settlement, wallets or deployment authority, and must be
> reported as `TEST_FIXTURE`, never as an unqualified PASS. A production review
> key, if any, will be published under a new trust-root version.
>
> **The fixture label is signer-declared, not a security control.** The
> `attester.key_class = TEST_FIXTURE_NOT_PRODUCTION` field is written by
> whoever signs, and only when they pass the opt-in `--test-fixture-key` flag.
> Anyone holding the committed fixture private key can omit the flag and get
> an unlabelled signature that reports a plain `PASS`. The label only helps
> cooperating signers tell fixture output apart. The security property comes
> from refusing this key identity outright in production (identity-based
> refusal of the fixture public key). Until that refusal is in force, treat
> every signature that verifies under the key below as a TEST FIXTURE
> signature, whatever its label says.

EverArcade OSC review key - TEST FIXTURE (Ed25519 SubjectPublicKeyInfo, base64):

```text
MCowBQYDK2VwAyEADvivFlH4llB2VHSZyKghEuygKr8qzGDnuzLxCLczZkg=
```

Fingerprint (SHA-256 over the exact base64 text above, no trailing newline):

```text
0989ae6991c6dc124ca674d5e6f19da4ee70299342ae3784abf7d529fe0a5db2
```

## Trust Boundary

This v1 trust-root document is **REVOKED** (`REVOKED_COMPROMISED_TEST_FIXTURE`; see the revocation notice at the top). The key above is the **TEST FIXTURE only**. It is not a trusted review key, and reviewers must not copy it as one. There is no trusted review key at present: `trust-roots/everarcade-review-root-v2.md` has status `PREPARED_NO_ACTIVE_KEY`.

When an ACTIVE trust-root version exists, reviewers must copy the review key only from that ACTIVE trust-root document as committed in this repository, or from a separately published copy of that exact document. Reviewers must never copy a key from generated build output, release output, a fixture under test, or any artifact whose authenticity is being evaluated.

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

> Result label: **TEST FIXTURE ONLY — not trust.** The key this command extracts is the revoked test fixture (see the revocation notice at the top of this document).

```bash
TRUSTED_PUBLIC_KEY="$(awk '/^```text$/{block++; next} block==1 && /^[A-Za-z0-9+/=]+$/{print; exit}' TRUST_ROOT.md)"
node creator-sdk/cli/everarcade.mjs world attest verify \
  --project examples/world-factory/frontier-settlement \
  --trusted-public-key "$TRUSTED_PUBLIC_KEY"
```
