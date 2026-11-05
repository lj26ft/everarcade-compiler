# EverArcade Trust-Root Revocations (append-only)

Entries are only ever appended. An entry is never edited, reordered or removed. A correction is a new entry that references the earlier one.

## Entry 1

- Trust-root version: `everarcade-review-root-v1`
- Status: REVOKED_COMPROMISED_TEST_FIXTURE
- Key: Ed25519 SubjectPublicKeyInfo, base64 `MCowBQYDK2VwAyEADvivFlH4llB2VHSZyKghEuygKr8qzGDnuzLxCLczZkg=`
- SPKI DER SHA-256:
  `6858d411f4f7ef9962de02945a7197ad9994903ec734627701cd1fb79968cd9c`
- Public key file: `fixtures/trust-root/test-attester-public-key.txt`
- Reason: test attester fixture; its private key is in the public repository, so anyone can sign with it.
- Affected: all attestations that verify under this key, whatever their timestamp.
- Last known-good attestation: none established by signature.
- Replacement: `trust-roots/everarcade-review-root-v2.md` (PREPARED_NO_ACTIVE_KEY).
- Date: prepared 2026-11-05 (CRED-01, owner decision t840ua0). Effective on owner-approved publication.
- Misuse: none known and none claimed. This entry records an exposure.
