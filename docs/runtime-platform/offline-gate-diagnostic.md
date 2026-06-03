# Runtime Offline Gate Diagnostic

## Purpose

The runtime offline gate verifies that `everarcade-runtime` can use the
repository's locked dependency graph without network access. This is the normal
release path for proving that the runtime can be checked and tested from the
committed lockfile and offline dependency source.

## Commands Tested

The diagnostic script runs these commands exactly:

```bash
cargo metadata --offline --locked
cargo check -p everarcade-runtime --offline --locked
cargo test -p everarcade-runtime --tests --offline --locked
```

## Known Current Failure

The current repository does not pass the normal offline runtime release path.
The captured diagnostic currently shows Cargo failing to resolve `bincode` from
the vendored directory source that replaces `crates-io`, which blocks metadata,
check, and test before runtime compilation begins. This PR is diagnostic-only:
it records which required offline commands fail and captures Cargo's output for
review.

## Diagnostic Report

Run the diagnostic with:

```bash
bash scripts/check_runtime_offline_gate.sh
```

The script writes its report to:

```text
reports/runtime_offline_gate_diagnostic.txt
```

The script exits nonzero if any checked command fails. A nonzero exit is
expected while the offline gate blocker remains unresolved.

## Scope Rule

Do not change `vendor/` in this PR. This task must not run `cargo vendor`,
regenerate dependencies, update `Cargo.lock`, or rename Cargo configuration.

## Next Phase

After this diagnostic identifies the blocker, a follow-up change can repair the
vendor/configuration state needed for the offline release gate.
