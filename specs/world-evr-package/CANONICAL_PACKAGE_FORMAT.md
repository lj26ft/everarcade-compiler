# Canonical Package Format

Canonical package bytes are protocol truth. For `WORLD_EVR_PACKAGE_SPEC_RC2`, the authoritative `world.evr` package hash is the `hash-manifest.json` recipe, not the older Rust envelope description below.

## Envelope

`CanonicalPackageEnvelope` fields are encoded in deterministic order:
1. `version`
2. `package_root`
3. `payload_root`
4. `replay_root`

## Versioning

Version is explicit and decoded through `PackageVersion { major, minor }`.
Unsupported versions are rejected.

## RC2 authoritative hash-manifest encoding

For RC2, `package_hash = sha256(package_hash_stream)`, where `package_hash_stream` is built by iterating `hash-manifest.files` sorted byte-lex by path and appending `path_utf8 || 0x00 || sha256(file_bytes)_hex || 0x0a` for each entry. Any Rust envelope or `execution-core/src/codec/package_encode.rs` description is non-authoritative for `world.evr` unless it produces byte-identical hash-manifest output.

## Legacy envelope encoding

The older envelope description below is retained as background for non-RC2 package work; it does not override the RC2 hash-manifest recipe.

## Decode semantics

Decoding requires full byte consumption (`decode_fully`).
Any trailing bytes, truncation, root mismatches, or malformed payload fails.

## Replay and root preservation

`replay_root`, `payload_root`, and `package_root` in the envelope must match
recomputed package values.

## Round-trip guarantees

`encode -> decode -> encode` must produce identical bytes.
