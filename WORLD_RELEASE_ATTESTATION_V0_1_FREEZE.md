# World Release Attestation V0.1 Freeze

## Status

`WORLD_RELEASE_ATTESTATION_V0_1` is the first frozen World Release Attestation protocol artifact. It is promoted from the V0.1 RC2 review candidate after independent review and live-world verification both reached `PASS` and all known review findings were resolved.

Future incompatible protocol changes MUST use `WORLD_RELEASE_ATTESTATION_V0_2` or a later schema version.

## What V0.1 Is

World Release Attestation V0.1 is a portable signed release statement for an EverArcade world artifact. A valid V0.1 attestation proves that:

- the signed statement was produced by the holder of the attester private key corresponding to an out-of-band trusted Ed25519 public key;
- the attested `world.evr` package hash re-derives from the supplied artifact;
- the World EVR Package V1 verifier passes for the supplied artifact;
- replay evidence independently re-derives the claimed runtime state, receipt, world, and continuity roots;
- deployment evidence matches independently re-derived roots and statuses;
- the attestation passes only when every signed `PASS` claim is reproduced by verifier-side evidence.

## Review History

| Stage | Result | Finding | Resolution |
| --- | --- | --- | --- |
| V0 | Not frozen | The embedded attester public key could act as a self-authenticating trust root. | V0.1 RC1 introduced mandatory out-of-band trusted-key verification. |
| V0.1 RC1 | Review candidate | Trusted-key gating rejected self-signed attestations, but `world_hash`, `continuity_root`, and runtime status claims still depended too heavily on supplied reports. | V0.1 RC2 required independent runtime root re-derivation from runtime artifacts and comparison against reports as evidence only. |
| V0.1 RC2 | Review candidate | No remaining protocol-blocking findings after RC2 root re-derivation review. | Independent review outcome: `PASS`; live-world verification outcome: `PASS`. |
| V0.1 | Frozen | All known findings resolved. | Freeze established here. |

## Findings and Fixes

### V0 Finding: Self-Authenticating Key

V0 allowed a verifier to rely on key material embedded in the attestation itself. That model could validate a malicious attestation signed by its own attacker-controlled key.

**Fix:** V0.1 requires a trusted Ed25519 public key supplied out of band. The verifier must compare that trusted key to `attester.public_key` before signature verification succeeds.

### RC1 Finding: Runtime Roots Were Not Fully Re-Derived

RC1 fixed trusted-key enforcement but still left runtime-derived claims vulnerable to over-trusting reports that could claim `PASS` without reproducing roots from the underlying runtime evidence.

**Fix:** RC2 requires verifier-side replay from genesis, deterministic receipt recomputation, runtime state comparison, journal comparison, receipt comparison, and root recomputation for `state_root`, `receipt_root`, `world_hash`, and `continuity_root`. Deployment reports are comparison evidence, not authority.

## Frozen Protocol Surface

The following V0.1 elements are frozen and MUST NOT change under the `WORLD_RELEASE_ATTESTATION_V0_1` protocol name:

- `schema_version`
- canonicalization rules
- signature rules
- trusted key requirements
- `world_hash` derivation
- `continuity_root` derivation
- verification requirements

Any change to those items is a breaking change and requires `WORLD_RELEASE_ATTESTATION_V0_2` or later.

## Allowed Additions in Future Versions

The following capabilities may be added by future versions without changing the V0.1 frozen artifact:

- attester registry
- XRPL identity anchoring
- Xahau identity anchoring
- multi-signature attestations
- key rotation metadata
- certificate chains

These additions must not be retroactively implied by V0.1 attestations.

## Breaking Changes Require V0.2

A future change requires `WORLD_RELEASE_ATTESTATION_V0_2` or later if it changes any frozen V0.1 rule, including but not limited to schema naming, canonical JSON behavior, signature input, trusted-key semantics, root derivation inputs, hash prefix normalization, package verification requirements, replay verification requirements, or remote verification requirements.

## Remaining Non-Goals

V0.1 does not provide:

- operator identity registry resolution;
- wallet identity binding;
- XRPL or Xahau anchoring;
- legal certification;
- key revocation;
- multi-party release approval;
- certificate-chain validation;
- automatic key rotation metadata.

## Freeze Rationale

V0.1 is frozen because the V0 self-authenticating key issue and the RC1 runtime-root authority gap are both resolved, the independent verifier can reproduce valid results from the review bundle, all listed failure fixtures are expected to fail, and the remaining items are explicitly non-goals or future-version additions rather than V0.1 correctness blockers.
