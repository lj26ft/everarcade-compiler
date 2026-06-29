# EverArcade Verification Classes

EverArcade artifacts MUST declare what kind of verification they support. The declaration is a scoped claim about what an independent verifier can prove from public data; it is not marketing language and it MUST NOT be upgraded by implication.

> Integrity does not imply truth.

A hash, signature, package root, or provenance chain can prove that bytes are unchanged and traceable. Those checks do not prove that the bytes describe an objectively true event unless the artifact is explicitly classified as `OBJECTIVE` and the value is forced by public inputs.

## `verification_class` field specification

All `.evr` artifacts and all package-local records that make externally visible claims MUST include a `verification_class` field.

| Property | Requirement |
| --- | --- |
| Field name | `verification_class` |
| Type | string enum |
| Allowed values | `OBJECTIVE`, `DETERMINISTIC`, `PROVENANCE` |
| Required for | Every `.evr` artifact manifest, proof record, root record, receipt stream descriptor, fixture descriptor, and package section that can be verified independently. |
| Default | No default. Missing fields MUST be rejected by strict verifiers for new artifacts. |
| Case | Uppercase ASCII exactly as listed. |
| Compatibility | Legacy artifacts that predate this field MAY be treated as `PROVENANCE` only when a verifier is explicitly running in a legacy compatibility mode. |

Example JSON:

```json
{
  "artifact_type": "world.evr.receipt_stream",
  "verification_class": "DETERMINISTIC",
  "root": "sha256:..."
}
```

Example TOML:

```toml
artifact_type = "world.evr.package"
verification_class = "DETERMINISTIC"
package_hash = "sha256:..."
```

## Class definitions

### `OBJECTIVE`

`OBJECTIVE` artifacts are truth-verifiable. The value is forced by public inputs, public rules, and deterministic verification steps. A verifier can recompute or derive the claimed value without trusting EverArcade, an operator, a creator, or a private witness.

Use `OBJECTIVE` only when all of the following are true:

1. every input needed to decide the claim is public;
2. the verification rule is deterministic and fully specified;
3. any conforming implementation must reach the same accept/reject result;
4. the claim is about the public input itself, not about a private off-chain fact.

Examples:

- canonical package hash computed from a complete public package tree;
- inclusion of a public receipt in a public commitment root;
- a public ledger transaction whose existence is checked against public ledger data.

### `DETERMINISTIC`

`DETERMINISTIC` artifacts are replay-verifiable. Given the same declared inputs, initial state, runtime profile, and canonicalization rules, conforming implementations produce the same outputs and roots.

Use `DETERMINISTIC` when the artifact proves reproducibility rather than objective truth. The verifier can say, “these inputs replay to these outputs,” but not necessarily, “the inputs were true observations of the external world.”

Examples:

- replay reports;
- deterministic simulation roots;
- fixture outputs under `fixtures/determinism/`;
- package execution outputs produced from declared genesis, receipt, contract, and runtime inputs.

### `PROVENANCE`

`PROVENANCE` artifacts provide integrity and provenance only. They bind bytes to authorship, custody, signature chains, timestamps, or package membership. They make no objective truth claim and no replay-completeness claim.

Use `PROVENANCE` when a verifier can confirm where bytes came from or whether bytes changed, but cannot independently derive the truth of their content.

Examples:

- human-authored metadata;
- screenshots, artwork, and descriptive assets;
- operator attestations that are not backed by public deterministic evidence;
- review notes and certification narratives whose signatures only identify the signer.

## Artifact classification registry

| Artifact or section | Required class | Rationale |
| --- | --- | --- |
| `world.evr` package manifest | `DETERMINISTIC` | The package inventory and package hash are replayable from declared package bytes. |
| `hash-manifest.json` / package hash stream | `OBJECTIVE` | The root is forced by public package bytes and a public hash algorithm. |
| world contract | `PROVENANCE` | Integrity and authorship can be checked; contract wisdom or fairness is not objectively proven. |
| RustRig manifest and source/build hashes | `PROVENANCE` | Hashes prove identity of code artifacts, not semantic truth by themselves. |
| genesis state | `DETERMINISTIC` | It is the public initial replay input; validity means reproducible loading and roots, not external truth. |
| receipt stream descriptor | `DETERMINISTIC` | Ordered receipts replay to deterministic roots when the stream is public and complete. |
| receipt inclusion proof against a public root | `OBJECTIVE` | Inclusion is forced by public proof bytes, public root, and public hashing rules. |
| replay certification report | `DETERMINISTIC` | It states that declared inputs reproduce declared outputs. |
| migration certification report | `DETERMINISTIC` | It states reproducible migration from source roots to target roots under declared rules. |
| root integrity report | `OBJECTIVE` when derived entirely from public bytes; otherwise `PROVENANCE` | Public roots are objectively recomputable; private attestations are provenance only. |
| operator identity registry | `PROVENANCE` | Registry hashes and signatures prove custody and change history, not external operator truth. |
| assets and asset manifests | `PROVENANCE` | Content hashes prove bytes and package membership only. |
| metadata and human-readable descriptions | `PROVENANCE` | Human descriptions are not replay or truth claims. |
| public determinism fixtures | `DETERMINISTIC` | Anyone can replay declared fixture inputs to expected roots. |

A narrower artifact MAY use a stronger class than its containing section only when the narrower claim is independently verifiable. For example, an asset manifest is `PROVENANCE`, while the SHA-256 digest of a public asset file is an `OBJECTIVE` byte-integrity claim.

## Mixed artifact decomposition guidance

Mixed artifacts MUST be decomposed so each independently verifiable claim has exactly one `verification_class`.

Do not label a whole artifact `OBJECTIVE` just because one field is objectively hashable. Instead:

1. split objective roots, deterministic replay outputs, and provenance statements into separate records; or
2. keep one envelope with a top-level `verification_class` for the envelope and a `claims[]` array where every claim has its own `verification_class`.

Recommended envelope pattern:

```json
{
  "artifact_type": "world.evr.certification_bundle",
  "verification_class": "PROVENANCE",
  "claims": [
    {
      "claim_type": "package_hash",
      "verification_class": "OBJECTIVE",
      "root": "sha256:..."
    },
    {
      "claim_type": "replay_result",
      "verification_class": "DETERMINISTIC",
      "final_root": "sha256:..."
    },
    {
      "claim_type": "reviewer_statement",
      "verification_class": "PROVENANCE",
      "signature": "..."
    }
  ]
}
```

Verifiers MUST report the class of each accepted claim. User interfaces MUST NOT collapse `PROVENANCE` into a green “verified truth” status.

## Verifier behavior

A conforming verifier MUST:

- reject unknown `verification_class` values;
- reject missing `verification_class` fields for new artifacts;
- preserve the class when forwarding, summarizing, or displaying verification results;
- avoid upgrading a claim from `PROVENANCE` to `DETERMINISTIC` or `OBJECTIVE` unless it independently verifies the stronger claim from public inputs;
- show `PROVENANCE` results as integrity/provenance confirmations only.

A conforming verifier SHOULD:

- explain which public inputs were used for `OBJECTIVE` and `DETERMINISTIC` claims;
- expose replay commands or fixture paths for `DETERMINISTIC` claims;
- include the phrase “Integrity does not imply truth” in operator-facing documentation and certification summaries.
