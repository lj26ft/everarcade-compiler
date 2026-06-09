# Offline Build Policy

## Vendor strategy

EverArcade intends to support reproducible Cargo builds through a vendored dependency directory referenced by `.cargo/config.toml`:

```toml
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "vendor"

[net]
offline = true
```

When `vendor/` is complete, release and bootstrap scripts should use frozen/offline Cargo modes and fail fast if required dependencies are absent.

## Current limitation: bincode/vendor issue

The current checkout contains an incomplete vendor snapshot. At audit time, the vendored source tree does not include all crates needed by the workspace; the known concrete failure is a missing `bincode` vendored crate during offline workspace resolution.

Impact:

- A full offline workspace build may fail even though local runtime proof scripts can succeed through temporary minimal workspaces or network-backed Cargo cache resolution.
- New contributors should not interpret the current `vendor/` directory as a complete release artifact.

## Approved workaround

Until the vendor artifact is restored:

1. Use the documented developer validation path first:

   ```bash
   CARGO_BUILD_JOBS=1 bash scripts/validate_developer_onboarding.sh
   ```

2. If a targeted Cargo command fails only because `.cargo/config.toml` forces the incomplete vendor directory, temporarily use a network-enabled Cargo configuration outside the repository or regenerate the vendor directory from a trusted networked environment.
3. Do not commit ad hoc partial vendor changes.
4. When preparing release/offline validation, restore a complete vendor artifact and verify checksums before claiming offline support.

## Restoring vendor artifacts

Preferred restoration paths:

```bash
bash scripts/vendor_deps.sh
```

or, when a release artifact exists:

```bash
bash scripts/restore_vendor_artifact.sh
```

Then verify with the relevant vendor preflight/release scripts before running offline builds.

## Claim boundary

Current classification for offline builds is **CONDITIONAL** until `vendor/` is complete and targeted offline Cargo checks pass from a clean clone.
