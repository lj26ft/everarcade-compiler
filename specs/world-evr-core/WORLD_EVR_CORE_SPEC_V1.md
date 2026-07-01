# WORLD_EVR_CORE_SPEC_V1

## Status

`WORLD_EVR_CORE_SPEC_V1` freezes the minimum `world.evr` container required to constitute a valid world. Core is the preservation, replay, migration, verification, restoration, and trust boundary. Implementations may add optional extended namespaces, but a package containing only Core sections is complete and valid when it satisfies this specification.

## Smallest valid world

The smallest valid world is a `world.evr/` directory containing exactly the Core namespace set below, with each namespace represented by a directory and with every load-bearing file listed in the package hash manifest.

```text
world.evr/
├── manifest/
├── runtime/
├── modules/
├── rigs/
├── fixtures/
├── proofs/
├── migrations/
├── restore/
├── trust/
└── signatures/
```

A valid Core world MUST answer:

1. what rules define this reality;
2. how the rules are executed;
3. how execution is replayed;
4. how replay and package integrity are verified;
5. how state is restored;
6. how the world is migrated; and
7. how trust and authenticity are established.

## Core design principles

Core MUST be minimal, deterministic, replayable, portable, preservation friendly, and changed only by an explicit Core specification version bump. Core data MUST NOT depend on renderer, commercial platform, social, federation, marketplace, or AI behavior for validity.

## Directory specifications

### `manifest/`

Purpose: human- and machine-readable world definition.

Required file: `manifest/world.json`.

Required fields:

- `core_spec`: MUST equal `WORLD_EVR_CORE_SPEC_V1`.
- `world_id`: stable world identifier.
- `world_name`: human-readable name.
- `version`: world version.
- `package_version`: package format version.
- `authors`: non-empty author list.
- `license`: license identifier or text reference.
- `description`: human-readable description.

Produced hash: `manifest_hash = sha256(canonical_tree(manifest/))`.

### `runtime/`

Purpose: deterministic execution requirements.

Required file: `runtime/runtime.json`.

Required fields:

- `runtime_version`.
- `abi_version`.
- `tick`: deterministic tick configuration.
- `memory_limits`.
- `fuel_limits`.

Produced hash: `runtime_hash = sha256(canonical_tree(runtime/))`.

### `modules/`

Purpose: executable deterministic logic.

Required file: `modules/modules.json`.

Required fields:

- `modules`: list of deterministic module descriptors.
- each module descriptor MUST include `module_id`, `version`, `kind`, `path`, and `sha256`.
- WASM modules and world contracts referenced by `path` MUST be present unless represented by a content-addressed external URI explicitly allowed by the verifier profile.

Produced hash: `module_hashes = sha256(canonical_tree(modules/))` plus per-module hashes listed in `modules/modules.json`.

### `rigs/`

Purpose: deterministic gameplay dependencies.

Required file: `rigs/rigs.json`.

Required fields:

- `rigs`: list of rig descriptors.
- each rig descriptor MUST include `rig_id`, `version`, `package_hash`, `invariant_version`, and `certification` metadata.

Produced hash: `rig_dependency_hash = sha256(canonical_tree(rigs/))`.

### `fixtures/`

Purpose: deterministic proof material for cold replay.

Required file: `fixtures/fixtures.json`.

Required fields:

- `action_logs`.
- `replay_fixtures`.
- `snapshots`.
- `expected_roots`.

Produced hash: `fixture_hash = sha256(canonical_tree(fixtures/))`.

### `proofs/`

Purpose: independent verification artifacts.

Required file: `proofs/proofs.json`.

Required fields:

- `state_root`.
- `receipt_root`.
- `continuity_root`.
- `world_hash`.
- `package_hash`.

Future proof systems, including recursive and zero-knowledge proofs, MAY be added as additional content-addressed proof artifacts without changing deterministic execution semantics.

### `migrations/`

Purpose: world portability.

Required file: `migrations/migrations.json`.

Required fields:

- `migration_metadata`.
- `migration_history`.
- `migration_proofs`.

Produced hash: `migration_hash = sha256(canonical_tree(migrations/))`.

### `restore/`

Purpose: reconstruction from archival material.

Required file: `restore/restore.json`.

Required fields:

- `checkpoints`.
- `snapshots`.
- `restore_metadata`.
- `restore_proofs`.

Produced hash: `restore_hash = sha256(canonical_tree(restore/))`.

### `trust/`

Purpose: trust, provenance, and review assumptions.

Required file: `trust/trust.json`.

Required fields:

- `attestations`.
- `certification_providers`.
- `review_history`.
- `known_assumptions`.

Produced hash: `trust_hash = sha256(canonical_tree(trust/))`.

### `signatures/`

Purpose: authenticity.

Required file: `signatures/signatures.json`.

Required fields:

- `developer_signatures`.
- `operator_signatures`.
- `certification_signatures`.

Produced hash: `signature_hash = sha256(canonical_tree(signatures/))`.

## Hash specifications

All Core hashes use SHA-256 over canonical bytes. JSON files MUST be canonicalized as UTF-8, sorted object keys, no insignificant whitespace, and normalized line endings before hashing when used in logical hashes. File-level package hashes MUST use exact bytes. Directory hashes MUST be computed as a byte-lexic stream of entries:

```text
for each file in directory subtree sorted byte-lex by relative path:
  relative_path_utf8 || 0x00 || sha256(file_bytes)_hex || 0x0a
```

The Core world hash is:

```text
world_hash = sha256(
  "WORLD_EVR_CORE_SPEC_V1" || 0x0a ||
  manifest_hash || 0x0a ||
  runtime_hash || 0x0a ||
  module_hashes || 0x0a ||
  rig_dependency_hash || 0x0a ||
  fixture_hash || 0x0a ||
  migration_hash || 0x0a ||
  restore_hash || 0x0a ||
  trust_hash || 0x0a ||
  signature_hash || 0x0a
)
```

The package hash MUST bind every load-bearing Core file and MAY bind Extended files in a separate extended hash manifest. A repaired package manifest MUST NOT make semantically mismatched Core data valid.

## Validation schema

The normative machine-readable starter schema is `schemas/world-evr-core-v1.schema.json`. Verifiers MAY implement stricter semantic checks, but MUST NOT accept a package that fails the required Core directory and field requirements.

## Compatibility rules

- Core v1 verifiers MUST understand every Core namespace listed in this document.
- Unknown future namespaces MUST be ignored but preserved unless a future specification explicitly marks them required.
- Extended namespaces MUST NOT be required to verify, replay, migrate, restore, or preserve Core.
- Existing V1 package hash-manifest rules remain valid as package-level binding rules when they produce the same load-bearing byte stream.

## Migration rules

A migration MUST preserve `world_id` continuity or explicitly declare a new world lineage. Migration proofs MUST bind source package hash, destination world ID, previous continuity root, and migration link hash. Migration failures MUST NOT alter the validity of the source Core package.

## Verifier requirements

A Core verifier MUST:

1. require all Core directories;
2. validate required files and fields;
3. recompute directory hashes;
4. recompute `world_hash` and `package_hash`;
5. validate runtime, module, rig, fixture, restore, migration, trust, and signature bindings;
6. ignore but preserve unknown optional namespaces; and
7. report Extended validation separately from Core validation.

## Preservation requirements

A Core-only package MUST be sufficient for offline verification, deterministic replay, migration, restoration, trust review, and long-term archival. If a package cannot be reconstructed without optional assets, projections, AI services, or commercial metadata, it is not a valid Core v1 world.
