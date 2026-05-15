# WASM Game ABI v0.1 Plan

## Guest input
- Canonical serialized `VmInput` bytes.
- Includes deterministic execution payload and previous state root context.

## Guest output
- Canonical serialized `VmOutput` bytes.
- Must include state changes and deterministic receipt material.

## State diff model
- Append-only `StateChange` list, deterministic ordering.
- No wall-clock, randomness, or host-global mutable dependencies.

## Receipt event
- Deterministic receipt id/hash derived from output payload.
- Receipt links checkpoint root and replay root for verification.

## Stdout event format
- Host emits protocol lines (see `docs/stdout-protocol.md`).
- Guest stdout is not a consensus surface in v0.1.

## Deterministic limits
- Bounded memory and payload sizes per execution.
- Fixed serialization format for input/output.
- No non-deterministic syscalls in guest.

## Disallowed host calls
- Direct network I/O from guest.
- Direct filesystem mutation from guest.
- Time/randomness APIs that bypass deterministic host mediation.
