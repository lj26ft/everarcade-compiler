# Proof Targets

## Property 1: Canonicalizer Determinism

```text
canonical(state)
=
canonical(state)
```

For all valid states, canonicalization is total, deterministic, and independent of host architecture, locale, map iteration order, filesystem order, wall-clock time, and serialization-library defaults.

## Property 2: Root Integrity

```text
state_root
=
SHA256(
canonical(state)
)
```

The digest input is exactly the canonical UTF-8 byte sequence with no prefix, suffix, newline, salt, or domain tag.

## Property 3: World Hash Integrity

```text
world_hash
=
SHA256(
state_root ||
receipt_root ||
continuity_root
)
```

Each root is decoded from lowercase hex to its 32-byte binary digest before concatenation. The input length is exactly 96 bytes.

## Property 4: Replay Equivalence

```text
replay(state)
=
live(state)
```

Replay equivalence means the replayed state and live state produce the same canonical bytes and root at the certified tick.

## Property 5: Migration Equivalence

```text
restore_elsewhere(state)
=
same canonical bytes
=
same root
```

Migration and restore preserve the canonical state commitment across environments.
