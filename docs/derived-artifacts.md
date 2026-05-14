# Derived Runtime Artifacts

EverArcade fixture binaries are **derived artifacts**.

## Source of truth

Protocol truth is defined by deterministic generation and validation:

- canonical fixture generation logic
- canonical package encoding
- canonical replay/checkpoint roots

## Non-authoritative outputs

Generated artifacts (for example `*.bin` fixture outputs) are:

- ephemeral
- reproducible
- rebuildable
- non-authoritative

They must not be treated as repository truth.

## Operator workflow

Generate fixtures into temporary paths (for example `mktemp`) before `run` commands and clean them up after execution.
