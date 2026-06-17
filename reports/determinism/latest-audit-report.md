# Arena Vanguard Determinism Audit Report

Generated: 2026-06-17T21:22:56Z

## Arena determinism tests
✔ deterministic input IDs are stable for identical canonical input (1.967011ms)
✔ serialization stability uses sorted canonical bytes regardless of insertion order (0.433574ms)
✔ root stability: replaying the same journal twice yields identical commitments (20.410361ms)
✔ replay equivalence: live execution and replay execution match all commitments (6.515773ms)
✔ persisted-state rebuild preserves commitments and roots (5.598784ms)
✔ different-machine simulation: independent roots with same journal match commitments (6.237362ms)
✔ genesis commitments are deterministic (0.380886ms)
ℹ tests 7
ℹ suites 0
ℹ pass 7
ℹ fail 0
ℹ cancelled 0
ℹ skipped 0
ℹ todo 0
ℹ duration_ms 200.898636

## Replay verification
HotPocket Arena Wrapper Live Path: PASS

## Evernode MCP audit
Evernode MCP CLI not found in PATH; external MCP audit must be run in the audit environment.

## Summary

HIGH: 0
MEDIUM: 0
LOW: documented environment and JSON-stringify findings only
