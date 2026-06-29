# World Package Specification v1

## Status

World Package Specification v1 defines `world.evr`, the canonical portable world artifact for EverArcade.

A World Package is the unit of ownership, deployment, migration, replay, certification, and restoration for a sovereign world. Developers ship one `world.evr`; operators inspect, verify, deploy, migrate, restore, replay, and certify that same artifact.

## Goals

The package format answers five operational questions with one artifact:

- **What do I ship?** A deterministic `world.evr` package.
- **What do operators run?** The runtime, contracts, RustRigs, genesis state, assets, policies, and proofs contained in the package.
- **What gets replayed?** Genesis state plus the continuity stream and receipt roots described by the package.
- **What gets migrated?** The world contract, certified RustRig references, state roots, continuity policy, and migration policy.
- **What gets certified?** The package manifest, contract declarations, RustRig integrity, genesis roots, replay reports, migration reports, equivalence reports, and root integrity reports.

## Package Artifact

The canonical artifact name is:

```text
world.evr
```

`world.evr` is a deterministic archive. Implementations may choose the physical container encoding, but the logical paths and hashes defined by this specification are canonical. The package hash is computed over the canonical package tree, excluding the mutable field that stores the package hash itself. Every `world.evr` package manifest MUST declare `verification_class = "DETERMINISTIC"` unless a future profile explicitly defines a narrower package envelope. Package-local proof and root records MUST follow the repository verification taxonomy in [`VERIFICATION_CLASSES.md`](../../VERIFICATION_CLASSES.md).

## Package Structure

A compliant `world.evr` contains the following top-level sections:

```text
world.evr
├── manifest/
├── world-contract/
├── rustrigs/
├── genesis/
├── continuity/
├── assets/
├── proofs/
└── metadata/
```

### `manifest/`

The manifest section contains the `WorldManifest`, the canonical inventory of the package.

Required file:

```text
manifest/world-manifest.toml
```

### `world-contract/`

The world contract section contains the sovereign rules for the world.

Required file:

```text
world-contract/world-contract.toml
```

The contract declares certified mutations, authorities, invariants, and RustRig references.

### `rustrigs/`

The RustRig section contains certified RustRig manifests and the metadata needed to bind executable world behavior to declared invariants.

Recommended files:

```text
rustrigs/inventory-rig.toml
rustrigs/combat-rig.toml
rustrigs/market-rig.toml
rustrigs/governance-rig.toml
```

### `genesis/`

The genesis section contains the starting state from which replay begins.

Required files:

```text
genesis/arena-state.json
genesis/initial-receipts.json
genesis/continuity-state.json
genesis/world-roots.toml
```

### `continuity/`

The continuity section contains policies that define how the world survives upgrades, epochs, restores, and retention boundaries.

Required file:

```text
continuity/policies.toml
```

### `assets/`

The assets section contains world assets, asset metadata, content hashes, and asset manifests. It MUST NOT contain executable code.

Required file:

```text
assets/asset-manifest.toml
```

### `proofs/`

The proof section contains certification and integrity reports.

Recommended files:

```text
proofs/root-integrity-report.txt
proofs/replay-certification-report.txt
proofs/migration-certification-report.txt
proofs/js-kernel-equivalence-report.txt
proofs/proof-map.toml
```

### `metadata/`

The metadata section contains non-authoritative package metadata for humans and tooling. Metadata MUST NOT override manifest, contract, genesis, continuity, asset, or proof content.

## WorldManifest

`WorldManifest` is the canonical manifest record for `world.evr`.

Required fields:

| Field | Type | Description |
| --- | --- | --- |
| `world_id` | string | Stable sovereign world identifier. |
| `world_name` | string | Human-readable world name. |
| `world_version` | semver string | Version of the packaged world. |
| `schema_version` | string | World Package schema version; `1` for this specification. |
| `verification_class` | enum string | Verification class for the package envelope; MUST be `DETERMINISTIC` for this specification. |
| `created_by` | string | Tool, account, organization, or authority that built the package. |
| `created_at` | RFC 3339 timestamp | Canonical package creation time. |
| `contract_version` | semver string | Version of the included world contract. |
| `runtime_version` | semver string | Runtime version targeted by the package. |
| `package_hash` | canonical hash string | Hash of the canonical package tree. |

Example:

```toml
world_id = "everarcade.reference.world"
world_name = "EverArcade Reference World"
world_version = "1.0.0"
schema_version = "1"
verification_class = "DETERMINISTIC"
created_by = "everarcade-cli"
created_at = "2026-06-18T00:00:00Z"
contract_version = "1.0.0"
runtime_version = "1.0.0"
package_hash = "sha256:pending-canonical-build"
```

## World Contract Section

`world-contract/world-contract.toml` contains:

- certified mutation declarations;
- authority declarations;
- invariant declarations;
- RustRig references.

Contract entries are authoritative. Runtime implementations MUST reject packages where a RustRig referenced by the contract is missing from `rustrigs/` or fails integrity verification.

## RustRig Section

The `rustrigs/` section contains:

- certified RustRig manifests;
- certification metadata;
- version references;
- invariant coverage declarations.

Each RustRig manifest identifies its package-local name, semantic version, source hash, certification hash, covered invariants, and referenced contract mutations.

## Genesis Section

The `genesis/` section contains:

- Genesis `ArenaState`;
- initial receipts;
- initial continuity state;
- initial world roots.

Genesis content is part of the replay boundary. A package is not replay certified until the genesis state, initial receipts, and initial roots can be loaded and replayed deterministically by the target runtime.

## Continuity Section

The `continuity/` section contains:

- continuity policy;
- migration policy;
- restore policy;
- retention policy;
- epoch policy.

These policies define how operators preserve world history, migrate compatible versions, restore from certified roots, retain receipts and proofs, and advance epoch boundaries.

## Assets Section

The `assets/` section contains:

- world assets;
- metadata;
- content hashes;
- asset manifests.

Assets are data only. The package MUST reject assets that contain executable code or scripts intended to modify runtime behavior. Runtime behavior belongs in certified RustRigs and world contracts.

## Proof Section

The `proofs/` section contains:

- root integrity reports;
- replay certification reports;
- migration certification reports;
- JS ↔ kernel equivalence reports;
- proof mapping references.

Proofs bind the package to certification evidence. Proof reports may be text, JSON, TOML, or another deterministic format, but proof references MUST be listed in `proofs/proof-map.toml`.

## World Package Invariants

### WP-001: Package Determinism

A canonical package build from identical inputs MUST produce identical logical content and the same `package_hash`. Integrity does not imply truth: package hashes and signatures prove reproducibility and byte integrity, but external truth claims require an explicit `OBJECTIVE` classification and public inputs that force the claimed value.

### WP-002: Manifest Integrity

The manifest MUST enumerate required package identity, version, runtime, contract, creation, and hash fields. Missing required fields invalidate the package.

### WP-003: World Contract Integrity

The world contract MUST be present, parseable, and internally consistent. All declared RustRig references MUST resolve to certified RustRig manifests.

### WP-004: RustRig Integrity

Every RustRig manifest MUST include version references, certification metadata, source or build hashes, and invariant coverage. Referenced invariants MUST exist in the world contract.

### WP-005: Genesis Integrity

Genesis `ArenaState`, initial receipts, initial continuity state, and initial world roots MUST be present and hash-consistent.

### WP-006: Proof Integrity

Proof mapping references MUST resolve to reports in `proofs/`, and those reports MUST match the roots, package hash, runtime version, and contract version they claim to certify.

### WP-007: Migration Portability

A package MUST include enough continuity and migration policy to move a certified world between compatible operators without changing world identity or replay roots.

### WP-008: Replay Reproducibility

Given the package genesis state, certified RustRigs, world contract, continuity state, and receipt stream, a conforming runtime MUST reproduce the certified roots.

## Certification Ladder

World Package certification advances through the following ladder:

```text
Package Built
↓
Package Verified
↓
Replay Certified
↓
Federation Certified
↓
Migration Certified
↓
Root Certified
↓
World Certified
```

- **Package Built**: the package tree exists and includes all required sections.
- **Package Verified**: manifest, contract, RustRigs, genesis, continuity, assets, and proofs pass structural validation.
- **Replay Certified**: deterministic replay from genesis reproduces expected roots.
- **Federation Certified**: federation deployment checks confirm compatible operator execution.
- **Migration Certified**: migration policy and migration reports prove portable upgrade or transfer.
- **Root Certified**: root integrity reports bind package content to certified world roots.
- **World Certified**: the complete package is accepted as the canonical portable world artifact.

## CLI Mapping

The EverArcade CLI maps package lifecycle operations to `world.evr`:

| Command | Package role |
| --- | --- |
| `everarcade init-world` | Create a world package skeleton. |
| `everarcade package-world` | Build deterministic `world.evr` from package inputs. |
| `everarcade inspect-world` | Display manifest, contract, RustRig, genesis, asset, continuity, and proof metadata. |
| `everarcade verify-world` | Validate package structure, hashes, invariants, and proof references. |
| `everarcade deploy-world` | Deploy a verified package to a target runtime/operator. |
| `everarcade migrate-world` | Execute or validate migration policy using certified package inputs. |
| `everarcade restore-world` | Restore from package genesis, continuity state, retained receipts, and certified roots. |

## Reference Package

A reference package skeleton is provided at `examples/reference-world-package/`. It demonstrates:

- Reference World Contract;
- Inventory RustRig;
- Combat RustRig;
- Market RustRig;
- Governance RustRig;
- Genesis State;
- Certification Artifacts.

## Conformance

A package conforms to World Package Specification v1 when it:

1. contains every required section;
2. includes a valid `WorldManifest` with `verification_class = "DETERMINISTIC"`;
3. includes a valid `world-contract/world-contract.toml`;
4. includes certified RustRig manifests for all contract references;
5. includes genesis state, receipts, continuity state, and roots;
6. includes continuity, migration, restore, retention, and epoch policies;
7. includes data-only assets with content hashes;
8. includes proof reports and proof mappings;
9. satisfies WP-001 through WP-008;
10. can progress through the certification ladder to World Certified.
