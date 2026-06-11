# EverArcade HotPocket Adapter v0.1

Canonical deterministic boundary between HotPocket inputs and EverArcade runtime artifacts.

Responsibilities:

- Accept HotPocket user input JSON.
- Normalize it into `everarcade.runtime.input.v0.1`.
- Execute deterministic `ping` and `join_player` mutations.
- Generate receipts, journals, checkpoints, and replay proofs.
- Persist canonical state roots under the configured adapter state directory.
- Reject unsupported actions and replayed canonical inputs.

The adapter has no npm runtime dependencies so it can be packaged reproducibly into a HotPocket contract bundle.
