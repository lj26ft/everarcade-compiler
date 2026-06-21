# Runtime Release Integrity

This guide documents deterministic runtime release validation for EverArcade.

## Deterministic artifacts

Use `scripts/hash_runtime_artifacts.sh` to generate canonical hashes for runtime files, replay outputs, package archive, and runtime release manifest.

Canonical deterministic files:

- `runtime/manifests/runtime-release-manifest.json`
- `runtime/manifests/runtime-artifacts.sha256`
- `runtime/manifests/replay.sha256`
- `dist/runtime.sha256`
- `dist/runtime-manifest.sha256`

Provenance metadata is emitted separately at `runtime/manifests/build-provenance.json` and includes a timestamp for human inspection only.

## Replay determinism

Run `scripts/verify_replay_determinism.sh` to generate replay output twice and verify `runtime/replay/latest/frame-0001.json` has a stable SHA-256 hash.

## Package integrity

Run `scripts/build_vm_runtime_appliance.sh` then `scripts/verify_runtime_package.sh`.

Package creation is hardened using stable archive ordering and normalized gzip headers.

## Manifest signing

Use:

- `scripts/sign_runtime_manifest.sh`
- `scripts/verify_runtime_manifest_signature.sh`

The signer uses OpenSSL detached signatures (`SHA-256` digest). If no key exists, a local RSA keypair is generated under `runtime/manifests/keys/` for local testing.

## Cross-machine verification

Run `scripts/cross_machine_repro_check.sh` on another Linux machine with the release artifacts. This validates:

- package hashes,
- replay hashes,
- runtime artifact hashes,
- required runtime files,
- manifest signature verification.

## Current reproducibility limitations

- Full cross-machine binary reproducibility depends on Rust toolchain and linker equivalence.
- Provenance includes `generated_at` and host metadata and is intentionally outside canonical deterministic hash scopes.
- Signature trust distribution is local by default (no PKI or remote trust root in this phase).
