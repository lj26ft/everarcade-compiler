# World Release Attestation V0.1 Review History

## Summary

World Release Attestation V0.1 reached freeze after three review stages: V0, V0.1 RC1, and V0.1 RC2. The final independent review verdict for RC2 was `PASS`, and live-world verification was `PASS`.

## V0 Review Record

- **Artifact:** `WORLD_RELEASE_ATTESTATION_V0`
- **Finding:** The attestation could be self-authenticating because the embedded public key could be treated as the verifier trust root.
- **Impact:** A forged release statement could include an attacker-controlled key and a matching signature.
- **Resolution status:** Resolved in V0.1 RC1 by requiring an out-of-band trusted public key.
- **Freeze status:** V0 was not frozen.

## V0.1 RC1 Review Record

- **Artifact:** `WORLD_RELEASE_ATTESTATION_V0_1_RC1`
- **Primary fix:** Mandatory trusted-key-gated Ed25519 signature verification.
- **Checks added:** missing trusted key rejection, wrong trusted key rejection, self-signed fake attestation rejection, tampered signed claims rejection, package hash re-derivation, and World EVR Package V1 verification.
- **Finding:** `world_hash`, `continuity_root`, replay status, and remote status were still too dependent on supplied runtime/deployment reports.
- **Reviewer verdict:** RC1 did not freeze; changes required.
- **Resolution status:** Resolved in V0.1 RC2 by requiring independent runtime root re-derivation.

## V0.1 RC2 Review Record

- **Artifact:** `WORLD_RELEASE_ATTESTATION_V0_1_RC2`
- **Primary fix:** Runtime roots and statuses are re-derived by the verifier from runtime artifacts, not trusted from reports.
- **Required evidence:** `world.evr/`, runtime state, journal, receipts, runtime report, deployment report, release attestation, and trusted public key.
- **Failure fixtures archived:** artifact package mismatch, artifact root mismatch, missing trusted key, modified continuity root, modified package hash, modified verification status, modified world hash, runtime journal tamper, runtime receipts tamper, runtime state tamper, self-signed fake attestation, and wrong trusted key.
- **Reviewer verdict:** `PASS`.
- **Live-world verification verdict:** `PASS`.
- **Resolution status:** Complete; no known protocol-blocking findings remain.

## Archived Finding Matrix

| Finding | Introduced / Observed | Fixed In | Status |
| --- | --- | --- | --- |
| Self-authenticating embedded key | V0 | V0.1 RC1 | Resolved |
| Missing trusted-key failure requirement | V0 | V0.1 RC1 | Resolved |
| Wrong trusted key rejection | V0 | V0.1 RC1 | Resolved |
| Package hash not sufficiently bound to artifact | V0 review scope | V0.1 RC1 | Resolved |
| Runtime reports over-trusted for roots/status | V0.1 RC1 | V0.1 RC2 | Resolved |
| Journal tamper not independently caught by root replay | V0.1 RC1 class | V0.1 RC2 | Resolved |
| Receipt tamper not independently caught by root replay | V0.1 RC1 class | V0.1 RC2 | Resolved |
| Runtime state tamper not independently caught by root replay | V0.1 RC1 class | V0.1 RC2 | Resolved |

## Reviewer Verdicts

- **RC1:** Changes required; trusted-key model accepted, runtime root re-derivation gap remained.
- **RC2:** `PASS`; trusted key enforcement, signature verification, package hash re-derivation, V1 package verification, runtime root re-derivation, replay status recomputation, remote status recomputation, and failure fixtures accepted.
- **Freeze:** `PASS`; V0.1 promoted to frozen protocol artifact.
