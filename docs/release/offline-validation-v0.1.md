# Offline validation v0.1

Vendor repair is performed from an authoritative checkout with:

```sh
cargo vendor
cargo metadata --offline --locked
CARGO_BUILD_JOBS=1 cargo test -p transport-core --offline --locked
CARGO_BUILD_JOBS=1 cargo test -p everarcade-cli lease --offline --locked
CARGO_BUILD_JOBS=1 cargo test -p everarcade-cli world --offline --locked
CARGO_BUILD_JOBS=1 cargo test -p everarcade-cli release --offline --locked
CARGO_BUILD_JOBS=1 cargo test -p everarcade-runtime transport --offline --locked
```

The repository `.cargo/config.toml` points crates.io to `vendor/` and enables offline mode. The repaired vendor tree includes the `bincode` crate required by `everarcade-abi`, so targeted offline tests no longer fail at dependency resolution.

Release packaging can include the vendor tree as `dist/vendor.tar.gz`; the CLI records its SHA-256 in `dist/vendor.tar.gz.sha256`, `dist/release-manifest.json`, and `reports/release/vendor-hash.txt`. The archive writer uses sorted paths so the hash is stable for an unchanged vendor tree.
