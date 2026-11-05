# Migration: trust-root v1 → v2 (operator guidance)

## What changes

- `everarcade-review-root-v1` (`TRUST_ROOT.md`) is **REVOKED**. Its only key is the committed test attester fixture, and the private key for it is public.
- `everarcade-review-root-v2` (`trust-roots/everarcade-review-root-v2.md`) is published with status **`PREPARED_NO_ACTIVE_KEY`**. It has no active review key yet.
- The fixture key is refused **by identity** in production, in `world attest create`, `world attest verify` and `world factory proof` / `remote-verify`. The verifier takes SHA-256 over the SPKI DER of the key presented at runtime and checks it against a denylist. File names and labels play no part.
- `world factory proof` / `remote-verify` no longer report `attestation_verification: PASS` from the attestation's embedded key alone. They report `UNTRUSTED_SELF_ATTESTED`, and the overall proof is `FAIL` (exit code 1) (H-2).

## Sequencing (each step needs owner approval; nothing here is activated by this document)

1. Option A: fixture labels, plus A-SUPP-01 (signer-declared label wording).
2. Option B: B-01 (identity refusal scaffold, `PRODUCTION_MODE`, exemption flags) **together with** the sealed B-02 fill, which puts the fixture SPKI DER SHA-256 on the denylist. Never put B-01 alone into production: with an empty denylist, production refuses every attestation create/verify (fail closed).
3. H-2: the proof path reports `UNTRUSTED_SELF_ATTESTED` instead of `PASS`.
4. Docs: the v1 REVOCATION NOTICE (in `TRUST_ROOT.md` and its byte-identical RC2 bundle copy), v2 `PREPARED_NO_ACTIVE_KEY`, `REVOCATIONS.md` and `ATTESTATION-STATUS-V2.md`.
5. Later, a separate decision: verifier version selection (`--trust-root`), downgrade refusal and a new attestation schema id carrying a signed `trust_root_version`.
6. Later, and **currently unauthorized**: the v2 key ceremony (generation, custody, signing, activation) and deployment.

## Production operators

- Until the key ceremony, production attestation verification **fails closed**. There is no trusted v2 key, and the fixture key is refused. This is intended.
- Do not use `--allow-test-fixture-attester` or `--allow-self-attested-test-only` in production. Both are refused there.
- Do not pass the v1 key (from `TRUST_ROOT.md`, `fixtures/trust-root/test-attester-public-key.txt` or `out/release/trusted-public-key.txt`) as `--trusted-public-key`. It is refused in production.
- Do not read existing `PASS` values in committed exports or in `live-deployment-proof.json` as trust-root-backed. See `ATTESTATION-STATUS-V2.md`.

## Non-production / RC2 reproduction

- RC2 CI (`scripts/ci/run-deterministic-world-factory.sh` and the two `rc2-fixture-*-must-fail.sh` scripts) keeps running in non-production with the explicit `--allow-test-fixture-attester` flag added by B-01. Must-fail fixtures still fail for their original reasons.
- Output from the fixture key is a TEST FIXTURE result, never a trust result.
- `world factory proof` in non-production also reports `UNTRUSTED_SELF_ATTESTED` (overall FAIL). No test-only flag turns it into `PASS`.

## Docs to re-word when the change set lands (not part of the code patches)

- `GATE_PROOFS.md:43` and `OPEN_SOURCE_CANDIDATE_RC2_REVIEW_BUNDLE/GATE_PROOFS.md:43` say: "The signer is the trusted review key from `TRUST_ROOT.md`." Reword to say this is the revoked test fixture.
- `EXPECTED_OUTPUTS.md:5`, the self-attested-fork README line 7, and the 4 `PASS` examples (Reviewer R-3 in CRED-01-OPTIONS-AB-REVIEW-01) need TEST FIXTURE wording.
- `OPEN_SOURCE_CANDIDATE_RC2.md:25` ("The trust root is `TRUST_ROOT.md` version `everarcade-review-root-v1`") needs to add that v1 is revoked and point to v2.
- `docs/founding-world-rc1/operator-guide.md:36` and `docs/world-factory-phase4.md:130` show `attestation_verification: PASS`. After H-2 the output is `UNTRUSTED_SELF_ATTESTED`, and the proof is FAIL until there is a trust-root-backed check.
- `PAYLOAD_BINDING.md:33` (and the bundle copy) says "re-signing with the trusted review key". Clarify that there is no trusted review key until v2 is active.
- `src-bin-everarcade/src/release.rs` writes `keys/trusted-public-key.txt`, which holds placeholder text, not a key. Mark it non-authoritative. This change is not in this set (no cargo).
- Any doc edits under the pin-checked paths (`README.md`, `TRUST_ROOT.md`, `OPEN_SOURCE_CANDIDATE_RC2.md`, `GATE_PROOFS.md`, `PAYLOAD_BINDING.md`, `REVIEW_TRUST_CHAIN.md` and `OPEN_SOURCE_CANDIDATE_RC2_REVIEW_BUNDLE/`) must not put a 40-or-more-hex string on the same line as "commit", "checkout", "pin" or "gate target", must not contain "same commit", and must not contain the stale prefix.

## Rollback

Revert the commits in reverse order (docs, then H-2, then B-02, then B-01, then A) or reverse-apply the patches. Nothing is deleted from history. Rolling back the docs restores v1 text without the revocation notice, but the key is still public, so a rollback does not make v1 trustworthy again.
