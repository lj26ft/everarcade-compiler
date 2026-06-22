# World Package Certification Framework v1

## Status

World Package Certification Framework v1 defines how a `world.evr` package becomes trusted, deployable, replayable, portable, and independently verifiable.

World Package Specification v1 defines **what a world package is**. This framework defines **how a world package becomes certified**.

Together they define the complete lifecycle:

```text
Create
Package
Certify
Deploy
Replay
Migrate
Verify
```

## Primary Artifact

The primary certification target is:

```text
world.evr
```

A World Package becomes eligible for deployment only after certification produces:

```text
WORLD PACKAGE CERTIFICATION: PASS
```

Uncertified packages are build artifacts only. They MUST NOT be treated as deployable, portable, replay-certified, or operator-trusted.

## Certification Model

Certification is a deterministic evidence pipeline:

```text
World Package
        ↓
Certification Pipeline
        ↓
Certification Report
        ↓
Certified World Package
```

The pipeline consumes a package, verifies each trust boundary, emits per-class reports, and produces one final certification report. The certified package is the same logical package plus certification evidence that binds the package hash, contract, RustRigs, genesis state, replay roots, federation outputs, migration outputs, root derivation, and proof artifacts.

## Trust Scope

Certification proves:

- package integrity;
- world contract integrity;
- RustRig integrity;
- genesis integrity;
- replay integrity;
- federation integrity;
- migration integrity;
- root integrity;
- proof integrity.

The certification framework is intentionally usable without reading runtime internals. Developers and operators should be able to answer:

- How do I certify a world?
- How do operators verify a world?
- How do I know a package is portable?
- How do I know a package is replayable?
- How do I know a package is safe to deploy?

## Certification Classes

Certification is divided into ten classes. A class may emit warnings for non-authoritative metadata, but any failure in required evidence blocks final certification.

### CLASS-A: Package Structure Certification

Verifies that `world.evr` contains the required logical sections:

```text
manifest
world-contract
rustrigs
genesis
continuity
assets
proofs
metadata
```

All required sections and required files from World Package Specification v1 MUST exist. Missing required structure fails certification before deeper semantic checks run.

Required output:

```text
package-structure-report.txt
```

### CLASS-B: Manifest Certification

Verifies the canonical manifest fields:

```text
world_id
world_version
schema_version
runtime_version
package_hash
```

The manifest MUST accurately describe package identity, package version, schema version, target runtime version, and canonical package hash. The `package_hash` MUST match package contents using the canonical package tree and MUST exclude only the mutable hash field according to the package specification.

Required output:

```text
manifest-report.txt
```

### CLASS-C: World Contract Certification

Verifies that the world contract declares and binds:

```text
mutation declarations
authority declarations
invariant declarations
RustRig declarations
```

The contract MUST conform to World Contract Framework v1. Each declared mutation MUST resolve to a declared RustRig. Each declared RustRig MUST resolve to a package-local RustRig manifest. Invariants MUST be named, stable, and referenced by the RustRigs that enforce or preserve them.

Required output:

```text
contract-report.txt
```

### CLASS-D: RustRig Certification

Verifies certified RustRig bindings, including the reference RustRigs:

```text
inventory.transfer()
combat.attack()
market.trade()
governance.vote()
```

The same class applies to future certified RustRigs. A package MAY use different mutation names when the world contract maps them explicitly, but the referenced RustRig certifications MUST be valid, package-local references MUST resolve, and certification hashes MUST bind to the declared runtime and contract scope.

Required output:

```text
rustrig-report.txt
```

### CLASS-E: Genesis Certification

Verifies:

```text
genesis state
initial roots
initial receipts
continuity state
```

Genesis is the replay boundary. The genesis state, initial receipts, and continuity state MUST produce deterministic initial roots. The roots emitted by certification MUST match `genesis/world-roots.toml` and the package proof mapping.

Required output:

```text
genesis-report.txt
```

### CLASS-F: Replay Certification

Verifies deterministic replay:

```text
same input
=
same state
=
same roots
```

Replay MUST be reproducible from package genesis, certified RustRigs, world contract declarations, continuity state, retained receipts, and runtime version. Replay certification fails if repeated execution produces divergent state, receipt, continuity, or world roots.

Required output:

```text
replay-report.txt
```

### CLASS-G: Federation Certification

Verifies that independent operators:

```text
operator A
operator B
operator C
```

produce identical:

```text
state_root
receipt_root
continuity_root
world_hash
```

Federation certification proves that package execution is not dependent on a single machine, operator, or hidden runtime side effect. Differences in logs, timestamps, or local paths are acceptable only if they are explicitly non-authoritative and excluded from canonical roots.

Required output:

```text
federation-report.txt
```

### CLASS-H: Migration Certification

Verifies migration continuity:

```text
source world
→ package
→ destination world
```

Migration MUST complete without state divergence, world identity loss, continuity root mismatch, or replay boundary breakage. Compatible schema and runtime constraints MUST be declared in continuity policy and reflected in the migration report.

Required output:

```text
migration-report.txt
```

### CLASS-I: Root Integrity Certification

Verifies canonical root derivation:

```text
ArenaState
→ canonical bytes
→ state root
→ world hash
```

Root integrity MUST use canonicalizer certification outputs. Any root used by deployment, replay, migration, federation, or proof mapping MUST be derived from canonical state bytes, canonical receipt bytes, canonical continuity bytes, and canonical package content.

Required output:

```text
root-integrity-report.txt
```

### CLASS-J: Proof Integrity Certification

Verifies:

```text
proof package
proof mapping
canonical fixtures
formal proof targets
```

Proof artifacts MUST match package contents, certification scope, package hash, runtime version, contract version, roots, and proof mapping. Proof integrity does not require every future formal proof to be complete, but every claimed proof target MUST be mapped, scoped, and reproducible.

Required output:

```text
proof-integrity-report.txt
```

## World Package Certification Invariants

### WP-CERT-001: Package Determinism

The same package inputs produce the same canonical package tree and the same package hash.

### WP-CERT-002: Manifest Integrity

The manifest accurately describes package contents, identity, versions, runtime target, and package hash.

### WP-CERT-003: World Contract Integrity

The declared contract matches package contents and all declared RustRig, mutation, authority, and invariant references resolve.

### WP-CERT-004: RustRig Integrity

Referenced RustRigs exist, are certified, and bind to declared contract mutations and invariants.

### WP-CERT-005: Genesis Integrity

Genesis state, initial receipts, continuity state, and initial roots are present and reproducible.

### WP-CERT-006: Replay Integrity

Replay remains deterministic for identical package inputs, receipt streams, runtime version, and contract declarations.

### WP-CERT-007: Federation Integrity

All participating operators produce identical `state_root`, `receipt_root`, `continuity_root`, and `world_hash`.

### WP-CERT-008: Migration Integrity

Migration preserves continuity, world identity, replayability, and canonical roots across source and destination worlds.

### WP-CERT-009: Root Integrity

Roots are derived from canonical state, canonical receipts, canonical continuity, and canonical package content.

### WP-CERT-010: Proof Integrity

Proof artifacts match the certification scope and package contents they claim to certify.

## Certification Reports

Certification writes reports to:

```text
reports/world-package-certification/
```

Required artifacts:

```text
package-structure-report.txt
manifest-report.txt
contract-report.txt
rustrig-report.txt
genesis-report.txt
replay-report.txt
federation-report.txt
migration-report.txt
root-integrity-report.txt
proof-integrity-report.txt
certification-report.txt
```

The final report MUST include the package identity, package hash, certification class results, certification ladder level, invariant coverage, and the required pass marker:

```text
WORLD PACKAGE CERTIFICATION: PASS
```

## CLI Mapping

Certification is a first-class CLI surface:

| Command | Certification role |
| --- | --- |
| `everarcade package-world` | Builds deterministic `world.evr` from package inputs. |
| `everarcade inspect-world` | Displays manifest, contract, RustRig, genesis, continuity, asset, proof, and certification metadata. |
| `everarcade verify-world` | Verifies structure, manifest hash, contract references, RustRig references, genesis roots, proof mappings, and root integrity. |
| `everarcade certify-world` | Runs CLASS-A through CLASS-J and emits certification reports. |
| `everarcade deploy-world` | Deploys only packages whose certification report passes policy. |

Deploy tooling MUST reject packages without a valid certification report unless explicitly running in a local unsafe development mode.

## Certification Ladder

Certification advances through nine levels:

| Level | Name | Meaning |
| --- | --- | --- |
| Level 0 | Package Built | Package inputs exist and `world.evr` can be produced. |
| Level 1 | Structure Certified | CLASS-A passes. |
| Level 2 | Contract Certified | CLASS-B, CLASS-C, and CLASS-D pass. |
| Level 3 | Replay Certified | CLASS-E and CLASS-F pass. |
| Level 4 | Federation Certified | CLASS-G passes across participating operators. |
| Level 5 | Migration Certified | CLASS-H passes for declared migration scope. |
| Level 6 | Root Certified | CLASS-I passes with canonicalizer outputs. |
| Level 7 | Proof Certified | CLASS-J passes against proof mappings and proof targets. |
| Level 8 | World Package Certified | All classes pass and final report contains `WORLD PACKAGE CERTIFICATION: PASS`. |

## Operator Verification Flow

Operators verify a package by checking:

1. `everarcade inspect-world world.evr` reports the expected `world_id`, version, runtime, and package hash.
2. `everarcade verify-world world.evr` passes structure, manifest, root, and proof checks.
3. `everarcade certify-world world.evr --reports reports/world-package-certification/` reproduces the certification reports or verifies their signatures.
4. `everarcade deploy-world world.evr` accepts only the certified package hash.

A package is portable when migration certification passes. A package is replayable when genesis and replay certification pass. A package is safe to deploy when all classes pass and policy accepts the resulting certification report.

## Reference Certified World

The canonical certification example lives at:

```text
examples/reference-certified-world/
```

It contains:

```text
reference world package
reference world contract
inventory RustRig
combat RustRig
market RustRig
governance RustRig
genesis state
proof artifacts
certification artifacts
```

This example is the reference layout for package authors, CLI implementers, and operators validating the certification workflow.

## Future Proof Mapping

Certification invariants map to future formal proof targets:

| Invariant | Future proof target |
| --- | --- |
| `WP-CERT-001` | package determinism proof |
| `WP-CERT-005` | genesis proof |
| `WP-CERT-006` | replay proof |
| `WP-CERT-007` | federation proof |
| `WP-CERT-008` | migration proof |
| `WP-CERT-009` | root integrity proof |

Related proof systems and artifacts:

- Proof Mapping Framework V1;
- Formal Proof Target Package;
- Canonicalizer Kernel.

## Conformance

A World Package is certified under this framework when:

1. CLASS-A through CLASS-J pass;
2. WP-CERT-001 through WP-CERT-010 are covered by reports;
3. package hash, roots, contract references, RustRig certifications, and proof mappings agree;
4. the package reaches Level 8 on the certification ladder;
5. the final certification report contains `WORLD PACKAGE CERTIFICATION: PASS`.


## RustRig Proof Target Package Candidates

World Package certification recognizes RustRig proof targets as package-cert candidates. A certified package may map `world.evr` mutation declarations to `proof-targets/rustrigs/` folders, the matching RustRig crate, invariant document, property target document, and certification report.

Certification chain:

```text
world.evr
→ certified RustRig kernels
→ signed certificate
→ independent re-check
→ deploy
```

The package certificate must name each certified RustRig kernel, its crate path, invariant list, receipt type, authority rule, and report path before deployment.
