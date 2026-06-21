# Repository Size Audit

## Before cleanup (2026-05-15)
- Total size: `70M`
- File count: `2199`
- Directory count: `272`

### Top 10 largest directories (depth=1)
1. `52M ./execution-core`
2. `15M ./.git`
3. `3.0M ./everarcade-host`
4. `196K ./test_vectors`
5. `132K ./docs`
6. `100K ./scripts`
7. `84K ./compiler`
8. `56K ./contracts`
9. `16K ./contract-api`
10. `16K ./everarcade-abi`

### Top 20 largest files
Top files were dominated by generated build outputs and Git pack data. Largest observed entries included:
- `./.git/objects/pack/pack-bdc8ad49780e79aec7220f4e23f07b60f26790a9.pack` (~15M)
- `./execution-core/target/release/deps/libsyn-...rlib` (~5.8M)
- `./execution-core/target/release/deps/libserde_derive-...so` (~3.6M)
- `./execution-core/target/release/deps/libsyn-...rmeta` (~2.2M)
- `./execution-core/target/release/deps/libproc_macro2-...rlib` (~1.1M)

### Generated folders found
- `./execution-core/target`
- No `.everarcade*` directories were present at audit time.

### Tracked large/generated files found
- `execution-core/target/**` was tracked in Git (build artifacts).
- Many tracked `.bin` fixtures/test vectors and `.wasm` outputs were present.

## Cleanup actions taken
- Removed generated local artifacts/directories:
  - `./target`
  - `./execution-core/target`
  - `./.everarcade`
  - `./.everarcade-smoke`
  - `./.everarcade-stress`
  - `./.everarcade-recovery`
  - `./tmp`
- Strengthened `.gitignore` coverage for runtime/generated outputs.
- Updated smoke/stress/recovery scripts to use temp state dirs in `/tmp` with cleanup traps.
- Added hygiene scripts:
  - `scripts/repo_size_audit.sh`
  - `scripts/clean_generated_artifacts.sh`
  - `scripts/check_no_generated_artifacts_tracked.sh`

## After cleanup (2026-05-15)
- Total size: `23M`
- File count: `1955`
- Directory count: `211`

### Largest remaining directories (depth=1)
- `15M ./.git`
- `3.9M ./execution-core`
- `3.0M ./everarcade-host`
- `196K ./test_vectors`

### Validation snapshot
- `find . -maxdepth 5 -type f -size +10M -print` now reports only Git pack data under `.git`.
- Working tree reflects deletion of tracked `execution-core/target/**` artifacts and script/hygiene updates.
