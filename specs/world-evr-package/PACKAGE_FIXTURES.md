# world.evr Package Fixtures RC2

Fixtures are intentionally small, text-based package directories so an independent verifier can inspect and recompute hashes without importing EverArcade runtime code.

## Authoritative RC2 hash recipe

1. Parse `hash-manifest.json`.
2. Reject unless `hash_alg` is `sha256`, `file_order` is `lexicographic-by-path`, and every `files[].path` is already sorted byte-lexicographically.
3. For each listed file, compute SHA-256 over exact file bytes and compare it to `files[].sha256`.
4. Build the package-hash input by concatenating each sorted entry as:

   ```text
   path_utf8 || 0x00 || sha256(file_bytes)_hex || 0x0a
   ```

5. SHA-256 hash that byte stream. The result must match `expected-package-hash.txt` and any archive-level expected package hash.
6. Reject unknown must-understand fields, missing required files, unlisted load-bearing files, and all explicit cross-artifact binding predicate failures.

## Mandatory semantic predicates

RC2 removes vague “mutually bound” wording. A valid package must satisfy all predicates in `WORLD_EVR_PACKAGE_SPEC_RC2.md`, including:

- manifest world, runtime, contract, and artifact hash bindings;
- restore checkpoint package identity and world identity bindings;
- recomputation of `restore/checkpoint.json.roots.continuity_root` from included restore accumulator data (`restore/journal.json` in these fixtures);
- proof/certification world and runtime bindings when present.

## Valid fixture

### `fixtures/world-package-valid-001/`

Contains:

- `manifest.json`
- `genesis/genesis.json`
- `runtime/runtime.json`
- `world-contract/world-contract.json`
- `hash-manifest.json`
- `expected-package-hash.txt`
- optional proof, checkpoint, and journal files

Expected result: **accept**. The verifier should recompute the package hash from `hash-manifest.json`, confirm every load-bearing file is listed, and confirm each mandatory predicate passes.

## Failure fixtures

- `failure-fixtures/wrong-world-id/` — manifest `world_id` is spoofed relative to bundled genesis data. Expected: **reject**.
- `failure-fixtures/manifest-runtime-mismatch/` — manifest runtime identity disagrees with bundled runtime identity. Expected: **reject**, even after hash-manifest repair.
- `failure-fixtures/genesis-hash-mismatch/` — manifest declares the wrong genesis digest. Expected: **reject**.
- `failure-fixtures/extra-unhashed-file/` — includes an unlisted runtime sidecar that could affect consensus. Expected: **reject**.
- `failure-fixtures/noncanonical-file-order/` — `hash-manifest.json` file entries are not lexicographically sorted. Expected: **reject**.
- `failure-fixtures/unknown-required-field/` — manifest contains an unknown must-understand consensus extension. Expected: **reject**.
- `failure-fixtures/missing-required-file/` — required world contract file is absent. Expected: **reject**.
- `failure-fixtures/restore-root-package-mismatch/` — restore checkpoint binds to the wrong root package. Expected: **reject**, even after hash-manifest repair.

Run the fixture harness with:

```text
node specs/world-evr-package/verify-package-rc2.mjs
```
