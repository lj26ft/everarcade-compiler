# EverArcade release packaging v0.1

Build a release-grade artifact set with:

```sh
everarcade release package
everarcade release verify
```

`everarcade release package` writes `dist/everarcade-cli`, `dist/everarcade-runtime`, `dist/world.evr`, `dist/runtime-bundle.zip`, `dist/release-manifest.json`, `dist/release-hash.txt`, optional `dist/vendor.tar.gz`, optional `dist/vendor.tar.gz.sha256`, and command reports. The release manifest records protocol, release id, package version, git commit, creation time, CLI hash, runtime binary hash, world package hash, runtime bundle hash, and optional vendor archive hash.

The release command builds the CLI and real `everarcade-runtime` target, initializes/packages a world if needed, builds a lease-specific HotPocket runtime bundle, computes SHA-256 hashes, and writes JSON reports under `reports/release/` and `reports/bundle/`.

Verification re-hashes every artifact, verifies `world.evr`, checks runtime bundle hash consistency, checks release manifest integrity through `release-hash.txt`, and validates the optional vendor archive hash.

`world.evr` is lease-independent sovereign world identity and continuity data. `runtime-bundle.zip` is lease-specific deployment data because it binds the verified world package to a selected lease id and HotPocket entrypoint.
