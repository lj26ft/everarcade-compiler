# EverArcade Trust Root v2

Trust-root version: `everarcade-review-root-v2`

Status: PREPARED_NO_ACTIVE_KEY

Supersedes: `everarcade-review-root-v1` (REVOKED_COMPROMISED_TEST_FIXTURE)

Minimum accepted version (production): `everarcade-review-root-v2`

Active review keys: none

Revoked keys (SPKI DER, base64): MCowBQYDK2VwAyEADvivFlH4llB2VHSZyKghEuygKr8qzGDnuzLxCLczZkg=

## Status meaning

`PREPARED_NO_ACTIVE_KEY` means this document is published, but no review key is active. Until a separately authorized key ceremony adds a key and changes the status to `ACTIVE`:

- production verification against this root fails closed: `FAIL: no active v2 review key`;
- no attestation is accepted as v2-backed;
- `TRUSTED_PUBLIC_KEY` values copied from `TRUST_ROOT.md` (v1) are revoked and must not be used for trust.

## Revoked keys

| Key | SPKI DER SHA-256 | Reason | Record |
|---|---|---|---|
| test attester fixture (`fixtures/trust-root/test-attester-public-key.txt`), base64 above | `6858d411f4f7ef9962de02945a7197ad9994903ec734627701cd1fb79968cd9c` | private key is in the public repository (test fixture) | `trust-roots/REVOCATIONS.md` entry 1 |

The fixture key is also refused by identity in production in every create, verify and proof path. The verifier takes the SPKI DER SHA-256 of the presented public key and refuses it if it is on the denylist. The match is on the key bytes, not on a file name or label.

## Acceptance rules (v2)

1. An attestation is v2-backed only if it carries a **signed** `trust_root_version` equal to `everarcade-review-root-v2` and verifies under an **active** v2 key. The rule needs a new attestation schema id, because the existing ids (`WORLD_RELEASE_ATTESTATION_V0`, `…_V0_1_RC1` and `…_V0_1_RC2`) are unversioned.
2. **No timestamp-based acceptance.** An attestation's timestamp is chosen by the signer. It is never used to accept, grandfather or reject a signature.
3. **No announcements signed with an old key.** Rotation and revocation announcements are never signed with, or authenticated by, a revoked or superseded key.
4. Downgrade is refused. In production, a missing or lower `trust_root_version`, selecting v1, or a root whose status is not `ACTIVE` all FAIL.
5. Existing committed attestations keep the status assigned in `trust-roots/ATTESTATION-STATUS-V2.md`. None is v2-backed.

## Rotation procedure (v2)

1. Make a new trust-root version document. Do not edit an active one in place.
2. A key ceremony (generation, custody, activation) must be authorized by a separate owner decision.
3. Publish the new public key, its SPKI DER SHA-256 and the new version string in a reviewed change.
4. The previous key stays accepted only for attestations that carry that key's signed `trust_root_version`, and only while the previous root's status allows it. Timestamps are not part of the rule.

## Revocation procedure (v2)

1. Append an entry to `trust-roots/REVOCATIONS.md`, giving the version, SPKI DER base64, SPKI DER SHA-256, reason and date.
2. Mark the affected trust-root document REVOKED with a notice at the top. Keep the document and its key text, and delete nothing.
3. Treat every attestation under the revoked key as invalid, whatever its timestamp. The last known-good attestation is "none established by signature" unless a separate, still-trusted signature establishes one.
4. Reviewers re-run attestation verification and the must-fail fixtures under the replacement root.

## Key ceremony

UNAUTHORIZED — requires a separate owner decision for generation, custody, signing, activation.

## Scope statement

This document records that the v1 key was exposed and has been revoked. It does not claim that the key was misused, and no misuse is known. Known deployed consumers of v1: none known. Live use of v1 elsewhere: unknown.
