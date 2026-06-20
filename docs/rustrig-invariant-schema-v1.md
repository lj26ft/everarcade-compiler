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

- `invariants.toml`: canonical mutation identity, state domains, mutation authority, and invariant rules.
- `properties.md`: verifier-readable property targets written as Given/When/Then claims.
- `certification.toml`: maturity, proof tier, invariant count, and verifier review metadata.

## Invariant Rule Schema

Each `[[invariants.rule]]` entry MUST declare:

| Field | Requirement |
| --- | --- |
| `id` | Stable ID such as `INV-001`. |
| `name` | Human-readable invariant name. |
| `category` | One allowed invariant category. |
| `description` | Verifier-readable claim. |
| `severity` | One allowed severity. |
| `proof_status` | One allowed proof status. |

Allowed categories: `Safety`, `Authorization`, `Conservation`, `Integrity`, `Continuity`, `Ordering`, `Economic`, `Governance`, `Ownership`, `Liveness`.

Allowed severity values: `Critical`, `High`, `Medium`, `Low`.

Allowed proof status values: `Unverified`, `Property-Tested`, `Differential-Tested`, `Certified`, `Formally-Proven`.

## Proof Harness Consumption Model

A verifier consumes `RustRig Source + Invariant Declaration + Property Targets`. The source supplies executable behavior, `invariants.toml` supplies the intended state domains and forbidden failures, and `properties.md` supplies testable claims. The verifier does not need to reverse-engineer intent from implementation code before building property tests, differential harnesses, formal models, or independent review packets.

## Certification Ladder

| Level | Name | Meaning |
| --- | --- | --- |
| 0 | Implemented | Code exists and is declared. |
| 1 | Property Tested | Properties exercise declared invariants. |
| 2 | Differential Tested | Independent implementation or oracle comparison exists. |
| 3 | Independent Review | External verifier reviewed declarations and evidence. |
| 4 | Certified | Certification metadata records approved status. |
| 5 | Formally Proven | Machine-checked proof covers the declared invariant set. |

## Registry Extension

Registry entries SHOULD include invariant IDs and proof status:

```json
{
  "mutation": "inventory.transfer",
  "maturity": "CERTIFIED",
  "invariants": ["INV-001", "INV-002", "INV-003"],
  "proof_status": "Independent Review"
}
```

## Success Criteria

A verifier should be able to answer what the RustRig mutates, what can go wrong, what must remain true, what is being proven, and what certification level exists without reading implementation code.
