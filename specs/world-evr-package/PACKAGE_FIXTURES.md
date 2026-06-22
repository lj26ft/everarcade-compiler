# world.evr Package Fixtures RC1

Fixtures are intentionally small, text-based package directories so an independent verifier can inspect and recompute hashes without importing EverArcade runtime code.

## Canonical hash recipe used by the fixture

1. Parse `hash-manifest.json` as canonical JSON.
2. Reject the package unless `file_order` is `lexicographic-by-path` and every `files[].path` is already sorted lexicographically.
3. For each listed file, compute SHA-256 over the exact file bytes and compare it to `files[].sha256`.
4. Build the package-hash input by concatenating each sorted entry as:

   ```text
   <path> NUL <sha256-hex> LF
   ```

5. SHA-256 hash that byte stream. The result must match `expected-package-hash.txt` and any archive-level package hash supplied by a `.evr` wrapper.
6. Reject unknown must-understand fields, missing required files, unlisted load-bearing files, and cross-artifact identity mismatches.

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

Expected result: **accept**. The verifier should recompute the package hash from `hash-manifest.json`, confirm every load-bearing file is listed, and confirm `world_id`, runtime, genesis, checkpoint, journal, and certification references are mutually bound.

## Failure fixtures

- `failure-fixtures/wrong-world-id/` — manifest `world_id` is spoofed relative to bundled genesis/contract data. Expected: **reject**.
- `failure-fixtures/manifest-runtime-mismatch/` — manifest runtime identity disagrees with bundled runtime/genesis identity. Expected: **reject**.
- `failure-fixtures/genesis-hash-mismatch/` — manifest declares the wrong genesis digest. Expected: **reject**.
- `failure-fixtures/extra-unhashed-file/` — includes an unlisted runtime sidecar that could affect consensus. Expected: **reject**.
- `failure-fixtures/noncanonical-file-order/` — `hash-manifest.json` file entries are not lexicographically sorted. Expected: **reject**.
- `failure-fixtures/unknown-required-field/` — manifest contains an unknown must-understand consensus extension. Expected: **reject**.
- `failure-fixtures/missing-required-file/` — required world contract file is absent. Expected: **reject**.
- `failure-fixtures/restore-root-package-mismatch/` — restore checkpoint binds to the wrong root package. Expected: **reject**.
