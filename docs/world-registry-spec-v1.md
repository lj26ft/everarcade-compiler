> **Repository boundary:** Registry material in this repository is limited to neutral world identity, metadata, and local validation examples.
>
> Hosted discovery, ranking, reputation, curation, and commercial registry operation are outside the scope of this open-source reference implementation.
>
# World Registry Specification v1

Certification output: `WORLD REGISTRY SPECIFICATION V1: PASS`

## 1. Objective

The **World Registry** is the canonical discovery layer for certified EverArcade worlds. It enables developers, operators, players, auditors, and tooling to discover certified worlds without affecting world ownership, execution, governance, or sovereignty.

The registry is a metadata layer. It is not a runtime, authority, marketplace, or world host.

## 2. Scope and Non-Goals

The registry provides discovery metadata for worlds, packages, operators, versions, and certification status.

The registry does **not**:

- Own worlds.
- Execute worlds.
- Modify worlds.
- Govern operators.
- Host packages or runtime state.
- Replace certification artifacts.
- Create marketplace authority.

## 3. Registry Principles

### Principle 1: Registry Does Not Own Worlds

The registry stores metadata only. World ownership remains with the world creator, world operator, or world governance as defined by the World Contract.

### Principle 2: Registry Does Not Execute Worlds

Execution remains with the operator runtime, Evernode infrastructure, or HotPocket federation.

### Principle 3: Registry Does Not Modify Worlds

Registry entries are informational. Registry publication cannot mutate state, receipts, continuity, roots, world contracts, or certification artifacts.

### Principle 4: Registry Must Be Portable

A registry implementation may be local, hosted, federated, mirrored, or self-hosted without changing registry semantics.

## 4. World Registry Entry

The canonical world discovery record is `WorldRegistryEntry`:

```rust
pub struct WorldRegistryEntry {
    pub world_id: String,
    pub world_name: String,
    pub world_version: String,
    pub world_hash: String,
    pub package_hash: String,
    pub package_version: String,
    pub contract_version: String,
    pub runtime_version: String,
    pub creator_id: String,
    pub description: String,
    pub tags: Vec<String>,
    pub published_at: String,
    pub updated_at: String,
    pub certification_level: u8,
    pub certification_status: String,
    pub operator_count: u32,
    pub latest_world_package: String,
}
```

### Field Semantics

| Field | Meaning |
| --- | --- |
| `world_id` | Stable identifier for world identity. It never changes across package versions. |
| `world_name` | Human-readable name for search and display. |
| `world_version` | World release version. Changes when the world package changes. |
| `world_hash` | Hash of reported world state. Changes when world state changes. |
| `package_hash` | Hash of certified world package contents. Changes when package contents change. |
| `package_version` | Version of the package referenced by this entry. |
| `contract_version` | World Contract version declared by the package. |
| `runtime_version` | Compatible runtime version or runtime range. |
| `creator_id` | Stable creator identifier. |
| `description` | Informational summary. |
| `tags` | Search labels. |
| `published_at` | First publication timestamp. |
| `updated_at` | Latest registry metadata update timestamp. |
| `certification_level` | Numeric registry certification level from 0 through 6. |
| `certification_status` | Human-readable proof/certification state. |
| `operator_count` | Count of discoverable operators for the world version. |
| `latest_world_package` | Canonical package reference tooling should deploy for this version. |

## 5. Identity Model

- `world_id` is the stable identifier and represents world identity.
- `world_version` changes when the world package changes.
- `package_hash` changes when world package contents change.
- `world_hash` changes when world state changes.

A world may therefore retain one `world_id` while exposing multiple versions, packages, and world-state hashes.

## 6. Registry Invariants

| Invariant | Name | Requirement |
| --- | --- | --- |
| WR-001 | Registry Determinism | Identical registry entries must produce identical registry hashes. |
| WR-002 | World Identity Stability | `world_id` remains stable across package versions. |
| WR-003 | Package Reference Integrity | Referenced package hashes must exist. |
| WR-004 | Certification Integrity | Certification status must match package certification artifacts. |
| WR-005 | Version Integrity | Latest version must reference newest certified package. |
| WR-006 | Operator Integrity | Operator records must reference valid worlds. |
| WR-007 | Metadata Integrity | Metadata must not contradict package metadata. |
| WR-008 | Package Hash Integrity | Published package hashes must match certified package hashes. |
| WR-009 | World Hash Integrity | Published world hashes must match reported world state. |
| WR-010 | Discovery Consistency | World lookup and search must return identical metadata. |

## 7. Registry Certification Levels

| Level | Name | Meaning |
| --- | --- | --- |
| 0 | Published | Metadata is published and discoverable. |
| 1 | Package Verified | Package hash and package reference are verified. |
| 2 | Replay Certified | Replay certification is surfaced. |
| 3 | Federation Certified | Federation certification is surfaced. |
| 4 | Migration Certified | Migration certification is surfaced. |
| 5 | Root Certified | Root integrity certification is surfaced. |
| 6 | World Package Certified | Complete world package certification is surfaced. |

## 8. Search API

### Search Worlds

Inputs:

- `tags`
- `creator`
- `operator`
- `world_name`
- `certification_level`

Output:

```rust
Vec<WorldRegistryEntry>
```

Search results must be derived from the same canonical entries returned by lookup APIs.

### Lookup World

Input: `world_id`

Output: `WorldRegistryEntry` for the current certified version.

### Lookup Version

Inputs: `world_id`, `world_version`

Output: `WorldRegistryEntry` for that exact version.

### Lookup Package

Input: `package_hash`

Output: `WorldPackageReference` containing package hash, package version, world id, world version, certification level, and deployable package locator.

## 9. Certification Integration

The registry must surface the existence and status of:

- Replay Certification
- Federation Certification
- Migration Certification
- Root Integrity Certification
- World Package Certification
- Proof Status

The registry exposes certification status without exposing implementation internals. Certification artifacts remain authoritative; registry entries mirror their discovery-relevant status.

## 10. Operator Discovery

Operator discovery records use `OperatorRegistryEntry`:

```rust
pub struct OperatorRegistryEntry {
    pub operator_id: String,
    pub world_id: String,
    pub runtime_version: String,
    pub verification_status: String,
    pub availability_status: String,
    pub last_seen: String,
}
```

Purpose: operator discovery only. Operator entries do not create operator governance, operator authority, or world ownership.

## 11. Registry Hash

`registry_hash` is computed from canonical registry entries using:

1. Canonical serialization.
2. UTF-8 byte ordering.
3. SHA-256.

The registry hash makes the registry replayable and auditable. A portable implementation must produce the same hash for identical canonical entries.

## 12. CLI Mapping

| CLI | Registry Action |
| --- | --- |
| `everarcade publish-world` | Publish or update a canonical world registry entry. |
| `everarcade search-worlds` | Search worlds by tag, creator, operator, name, or certification level. |
| `everarcade inspect-world` | Lookup current or versioned world metadata. |
| `everarcade inspect-package` | Lookup package reference and certification status. |
| `everarcade inspect-operator` | Lookup operator discovery metadata. |

## 13. Reference Registry

A reference registry is provided at `examples/reference-world-registry/`. It contains Arena Vanguard, Iron Frontier, and Sovereign City; multiple versions per world; multiple operators per world; mixed certification levels; and a generated example registry hash.

## 14. Registry Reports

Registry reports are provided at `reports/world-registry/`:

- `registry-structure-report.txt`
- `world-entry-report.txt`
- `operator-entry-report.txt`
- `version-report.txt`
- `certification-report.txt`
- `registry-hash-report.txt`
- `report.txt`

## 15. Future Extensions

Future registry capabilities may include World Categories, Featured Worlds, Governance Discovery, World Reputation, Marketplace Discovery, Cross-World Discovery, and Federated Registries.

These extensions are informational only and must not affect sovereignty.

## 16. Success Criteria

A player can answer what worlds exist, which worlds are certified, who operates them, what version is current, and what package should be deployed.

A developer can answer how to publish a world, update a world, and expose certification status.

An operator can answer which worlds can be hosted, which packages are certified, and which runtimes are compatible without reading package contents or runtime internals.

The World Registry completes the first end-to-end world lifecycle: Create, Package, Certify, Register, Discover, Deploy, Operate, Verify, Migrate, Continue.
