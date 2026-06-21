# Reproducible Linux Build Policy

- Supported target: `x86_64-unknown-linux-gnu`.
- Rust toolchain must be pinned and reproducible.
- Build commands run with `--offline --locked --frozen` where applicable.
- `Cargo.lock` and `vendor/` are mandatory for release rebuilds.
- Tarball reproducibility uses sorted file order, normalized metadata, and deterministic manifest hashes.
