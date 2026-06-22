# world.evr Package Fixtures V1

These fixtures exercise the frozen `WORLD_EVR_PACKAGE_SPEC_V1` package artifact format.

## Authoritative V1 hash recipe

For V1, `package_hash = sha256(package_hash_stream)`, where `package_hash_stream` is built by iterating `hash-manifest.files` sorted byte-lex by path and appending:

```text
path_utf8 || 0x00 || sha256(file_bytes)_hex || 0x0a
```

The package hash is reproducible from package contents and the public spec only. No EverArcade runtime code is required.

## Canonical runtime version field

`runtime_version` is the canonical runtime version field in both `manifest.runtime.runtime_version` and `runtime/runtime.json.runtime_version`.

Compatibility aliases are non-canonical and should not appear in V1 fixtures.

## Valid fixture

- `fixtures/world-package-valid-001/` — valid V1 package fixture. It binds manifest, genesis, runtime, world contract, restore checkpoint, restore journal, certification proof, hash manifest, and expected package hash.

## Failure fixtures

Each directory under `failure-fixtures/` must fail V1 verification:

- `continuity-root-mismatch/` — fails because `checkpoint.roots.continuity_root` does not equal the recomputed continuity root.
- `extra-unhashed-file/` — fails because a load-bearing file is present outside `hash-manifest.json`.
- `genesis-hash-mismatch/` — fails because the manifest genesis hash does not match `genesis/genesis.json`.
- `manifest-runtime-mismatch/` — fails because manifest runtime identity does not match `runtime/runtime.json`.
- `missing-required-file/` — fails because a required file is absent.
- `noncanonical-file-order/` — fails because `hash-manifest.files` is not byte-lexicographically sorted by path.
- `restore-root-package-mismatch/` — fails because restore checkpoint/journal package binding does not match the manifest package identity.
- `unknown-required-field/` — fails because an unknown must-understand field is present.
- `wrong-world-id/` — fails because manifest world identity does not match genesis world identity.

## Verification command

```bash
node specs/world-evr-package/verify-package-v1.mjs
```
