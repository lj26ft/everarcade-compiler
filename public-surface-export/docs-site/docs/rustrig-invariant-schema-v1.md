# RustRig Invariant Declaration Schema v1

RustRig certification is declaration-driven: a RustRig is certifiable only when it explicitly declares its mutation definition, state model, invariant rules, property targets, and certification metadata.

## Canonical RustRig Layout

```text
crates/rustrigs/<domain>/
README.md
invariants.toml
properties.md
certification.toml
src/
```

## Required Declaration Files

- `invariants.toml`: canonical mutation identity, state domains, mutation authority, implementation hash, and invariant rules.
- `properties.md`: verifier-readable property targets written as Given/When/Then claims.
- `certification.toml`: maturity, proof tier, invariant count, implementation hash binding, and verifier review metadata.

## Invariant Classes

Certified RustRigs MUST separately declare both INTEGRITY and SAFETY invariant sections: Integrity Invariants and Safety Invariants.

- **Safety** invariants are domain safety obligations: conservation, authorization, bounds, atomic rejection, no double spend, tally correctness, and other properties that prevent valid-looking deterministic execution from violating game rules.
- **Integrity** invariants are execution-integrity obligations: deterministic behavior and consistency of receipts, replay, and state roots.
- **Replay**, **Root**, **Receipt**, and **Determinism** are integrity sub-classes that MAY be used when a declaration needs machine-reviewable specificity.

Allowed categories: `Safety`, `Integrity`, `Replay`, `Root`, `Receipt`, `Determinism`.

## Invariant Rule Schema

Each `[[invariants.rule]]` or `[[rustrig.invariants.rule]]` entry MUST declare:

| Field | Requirement |
| --- | --- |
| `id` | Stable ID such as `INV-SAFE-001`. |
| `name` | Human-readable invariant name. |
| `category` | One allowed invariant category. |
| `description` | Verifier-readable claim. |
| `severity` | One allowed severity. |
| `verification_method` | One allowed verification method. |
| `proof_status` | One allowed proof status for this invariant. |
| `artifact_hash_binding` | Exact `sha256:<hash>` implementation artifact hash this invariant evidence binds to. |

Allowed severity values: `Critical`, `High`, `Medium`, `Low`.

Allowed verification method values: `differential`, `replay`, `property-test`, `formal-proof`, `manual-review`, `independent-review`, `zk-target`.

Allowed proof status values: `Unverified`, `Property-Tested`, `Differential-Tested`, `Independently-Reviewed`, `Certified`, `Formally-Proven`, `ZK-Targeted`.

`verification_method` records how the invariant is discharged, such as integrity/differential checking, safety property testing, formal proof, independent review, or future ZK proof.

`proof_status` is per invariant. A RustRig-level certification status MUST NOT obscure which individual claims are unverified, property-tested, independently reviewed, certified, formally proven, or ZK-targeted.

## Certification Metadata Schema

Certification metadata MUST bind to the exact implementation artifact:

```toml
implementation_hash = "sha256:<hash>"
certification_status = "void_if_hash_changes"
```

Void-on-modify rule: If the implementation hash changes, certification is void until re-certified.

## Proof Harness Consumption Model

A verifier consumes `RustRig Source + Invariant Declaration + Property Targets + Certification Metadata`. The source supplies executable behavior, invariant declarations supply intended safety and integrity obligations, properties supply testable claims, and certification metadata binds evidence to an implementation hash. The verifier does not need to reverse-engineer intent from implementation code before building property tests, differential harnesses, formal models, ZK targets, or independent review packets.

## Certification Ladder

| Level | Name | Meaning |
| --- | --- | --- |
| 0 | Implemented | Code exists and is declared. |
| 1 | Property Tested | Properties exercise declared invariants. |
| 2 | Differential Tested | Independent implementation or oracle comparison exists. |
| 3 | Independent Review | External verifier reviewed declarations and evidence. |
| 4 | Certified | Certification metadata records approved status for the bound implementation hash. |
| 5 | Formally Proven | Machine-checked proof covers the declared invariant set. |
| 6 | ZK Targeted | Invariant is included in a future zero-knowledge proof target. |

## Registry Extension

Registry entries SHOULD separate integrity and safety invariant IDs and include implementation hash binding:

```json
{
  "mutation": "inventory.transfer",
  "maturity": "CERTIFIED",
  "integrity_invariants": ["INT-DET-001", "INT-REC-001", "INT-REP-001", "INT-ROOT-001"],
  "safety_invariants": ["INV-SAFE-001", "INV-SAFE-002"],
  "implementation_hash": "sha256:<hash>",
  "certification_status": "void_if_hash_changes"
}
```

## Success Criteria

A verifier should be able to answer what integrity claims are made, what safety claims are made, how each claim is verified, the current proof status for each claim, which implementation hash the certification binds to, and whether changing the implementation voids certification without reading implementation code.
