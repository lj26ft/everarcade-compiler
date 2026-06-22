# World Package Specification V1


WORLD_EVR_PACKAGE_SPEC_V1 is the frozen v1 package artifact format for world.evr packages.

## Status

`WORLD_EVR_PACKAGE_SPEC_V1` is the frozen v1 package artifact format for `world.evr` packages. It freezes the RC2 fixes for the RC1 binding gaps found by independent verification.

## Authoritative Package Hash

For V1, `hash-manifest.json` is authoritative. Older Rust envelope or `execution-core/src/codec/package_encode.rs` descriptions are non-authoritative for `world.evr` unless they are rewritten to produce byte-identical `hash-manifest.json` output and the same package hash stream.

```text
package_hash = sha256(package_hash_stream)

package_hash_stream =
for each file in hash-manifest.files sorted byte-lex by path:
  path_utf8 || 0x00 || sha256(file_bytes)_hex || 0x0a
```

The verifier MUST parse `hash-manifest.json`, verify the path order and listed file hashes, build the stream exactly as shown, and compare the digest to `expected-package-hash.txt` or an externally supplied expected package hash.

## Mandatory Binding Predicates

A package is valid only if **all** mandatory predicates pass. Manifest hashing alone is not a semantic binding predicate; a repaired hash-manifest MUST NOT make a semantically mismatched package valid.

### Package File Binding

```text
Every file path in hash-manifest.files exists.
Every file hash equals sha256(file_bytes).
hash-manifest.files is sorted byte-lex by path.
No load-bearing file exists outside hash-manifest.files.
package_hash matches expected-package-hash if provided.
```

`hash-manifest.json` and `expected-package-hash.txt` are verifier control files and are not entries in `hash-manifest.files` for the V1 directory fixtures.

### Manifest Binding

V1 fixtures use the JSON schema in `fixtures/world-package-valid-001/manifest.json`. The canonical package identifier is `manifest.package_name`.

```text
manifest.package_name == expected package name / artifact name.
manifest.world_id == genesis/genesis.json.world_id.
manifest.runtime.runtime_id == runtime/runtime.json.runtime_id.
manifest.runtime.runtime_version == runtime/runtime.json.runtime_version.
manifest.world_contract.contract_id == world-contract/world-contract.json.contract_id.
manifest.genesis.sha256 == sha256(genesis/genesis.json).
manifest.runtime.sha256 == sha256(runtime/runtime.json).
manifest.world_contract.sha256 == sha256(world-contract/world-contract.json).
```

If future production manifests add runtime identity fields, each load-bearing field MUST be explicitly bound. For example:

```text
manifest.runtime.runtime_hash == sha256(runtime/runtime.json)
```

A verifier MUST reject a package when `manifest.runtime.runtime_id` differs from `runtime/runtime.json.runtime_id` or when `manifest.runtime.runtime_version` differs from `runtime/runtime.json.runtime_version`, even after rebuilding `hash-manifest.json`.

### Restore Binding

```text
restore/checkpoint.json.world_id == manifest.world_id.
restore/checkpoint.json.root_package == manifest.package_name.
restore/journal.json.root_package == manifest.package_name.
restore/journal.json.from_checkpoint == restore/checkpoint.json.checkpoint_root
  OR restore/journal.json.checkpoint_root == restore/checkpoint.json.checkpoint_root.
restore/checkpoint.json.roots.continuity_root == sha256(restore/journal.json bytes).
```

V1 fixture continuity recomputation is intentionally cold and package-local: `restore/journal.json` is the included restore accumulator data, and the continuity root is the SHA-256 digest of its exact bytes with the `sha256:` prefix. No stored continuity root may be accepted as trusted truth without this recomputation.

A verifier MUST reject a package when `restore/checkpoint.json.root_package` differs from `manifest.package_name`, even after rebuilding `hash-manifest.json`.

### Migration Binding

If migration artifacts exist in a package, the verifier MUST enforce:

```text
migration.source.package_hash == package_hash of source package.
migration.destination.world_id == manifest.world_id.
migration.continuity.previous_root == source continuity root.
migration.link_hash == sha256(canon(migration_link)).
```

V1 fixtures do not include migration artifacts; these predicates are mandatory for any package that does.

### Proof / Certification Binding

If `proof/certification.json` exists, the verifier MUST enforce:

```text
proof/certification.json.world_id == manifest.world_id.
proof/certification.json.runtime_id == manifest.runtime.runtime_id.
```

Detached or archive-level certification that is outside `hash-manifest.files` MUST additionally bind:

```text
proof/certification.json.package_hash == package_hash.
```

Inline certification files included in `hash-manifest.files` cannot store their own final package hash without self-reference. For inline V1 fixture certification, `expected-package-hash.txt` is the package-hash binding control.

## Failure Fixtures

The V1 verifier MUST accept `fixtures/world-package-valid-001/` and reject every directory under `failure-fixtures/`. In particular:

- `failure-fixtures/manifest-runtime-mismatch/` fails because manifest runtime identity disagrees with the bundled runtime.
- `failure-fixtures/restore-root-package-mismatch/` fails because the restore checkpoint root package disagrees with package identity.
- `failure-fixtures/continuity-root-mismatch/` fails because the restore checkpoint continuity root does not equal the recomputed continuity root.

These two failures are semantic binding failures and MUST remain failures after a hash-manifest repair pass.

## Runtime version field canonicalization

`runtime_version` is the only canonical runtime version field in V1. Compatibility aliases are non-canonical and MUST NOT appear in V1 fixtures. Verifiers MAY report a targeted compatibility error for aliases, but V1 validation requires `manifest.runtime.runtime_version == runtime/runtime.json.runtime_version`.
