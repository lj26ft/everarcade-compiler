# Arena Vanguard Runtime Flow

1. `everarcade run-arena-local` seeds the runtime directories and installs the Arena Vanguard package.
2. Genesis starts at tick `0` with no players.
3. Serialized inputs are sorted by `(tick, player)`.
4. Join creates or reconnects a player.
5. Move applies deterministic fixed-point deltas.
6. Attack validates Manhattan range, applies fixed damage, and emits damage/death events.
7. Each input writes a receipt with pre-state root, input hash, post-state root, and event names.
