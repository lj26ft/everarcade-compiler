# Adapter 01 → Adapter 02 public contract diff

This comparison is limited to world.evr-consumed, load-bearing boundaries.

| Boundary | Classification | Adapter 02 / RC1 disposition |
|---|---|---|
| Authority bootstrap | SEMANTICALLY_COMPATIBLE_EVOLUTION | Uses RC1 authenticated bootstrap and READY; no adapter bypass |
| Authenticated READY | NEW_REQUIRED_CONTRACT | RC1's authenticated READY is required before consumption |
| Owner / generation | NEW_REQUIRED_CONTRACT | Preserved and validated by the RC1 provider boundary |
| Canonical root | SEMANTICALLY_COMPATIBLE_EVOLUTION | Remains provider-governed and is not caller asserted |
| Storage identity | SEMANTICALLY_COMPATIBLE_EVOLUTION | Exact physical/root identity remains load-bearing |
| Current-head access | SEMANTICALLY_COMPATIBLE_EVOLUTION | RC1 provider current head is authoritative |
| Governed execution | UNCHANGED | Same world consumer API profile; exact RC1 identity changes |
| Persistence | SEMANTICALLY_COMPATIBLE_EVOLUTION | RC1 durable commit/publication contract is consumed unchanged |
| Replay | UNCHANGED | Production replay remains deterministic and authority-bound |
| Recovery | SEMANTICALLY_COMPATIBLE_EVOLUTION | RC1 recovery validates current head and storage identity |
| Portable reconstitution | NEW_REQUIRED_CONTRACT | Exact governed operation is now explicitly bound |
| Operation identity | BINDING_UPDATE_REQUIRED | `everarcade.graph.current-head-reconstitution.v1` is mandatory |
| Error / rejection semantics | SEMANTICALLY_COMPATIBLE_EVOLUTION | Fail-closed RC1 rejections propagate through the adapter |
| Compatible runtime set | ADAPTER_UPDATE_REQUIRED | Major V2 exact set replaces A1 identity for new V2 packages |

No boundary is removed. The public world-runtime API remains
`everarcade.world-runtime.graph-consumer@1.0.0`; the major adapter successor is
required because the immutable compatible runtime set and explicit authority /
portable-state intake requirements changed.
