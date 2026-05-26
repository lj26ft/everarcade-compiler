# Sovereign Runtime Architecture

EverArcade transitions to a hybrid sovereign runtime with four layers: deterministic core runtime (authoritative law), GPU execution boundary (non-authoritative assist), XRPL integration (ownership/settlement anchoring), and Evernode orchestration (tenant scheduling and hosting).

## Layer rules
1. CPU deterministic runtime is authoritative for tick/state/settlement continuity.
2. GPU outputs are replay-linked witnesses and can never mutate authority directly.
3. XRPL integration is deterministic object-model and witness based in this phase (no live networking).
4. Evernode orchestration exports validation roots and world continuity proofs.
