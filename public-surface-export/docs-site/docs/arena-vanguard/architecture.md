# Arena Vanguard Architecture

Arena Vanguard is the canonical first playable World Contract package. The authoritative path is deterministic runtime execution over serialized inputs; the browser projection only reads verified state.

```mermaid
flowchart LR
  Input[Join/Move/Attack/Disconnect] --> Runtime[EverArcade Runtime]
  Runtime --> Contract[Arena Vanguard World Contract]
  Contract --> State[State Root]
  Contract --> Receipts[Receipt Root]
  State --> Replay[Replay]
  Receipts --> Replay
  Replay --> Verify[Verification]
```

State uses fixed tick ordering, fixed-point `i32` coordinates in centimeters, player health, combat events, and tick counters. No floating-point simulation is used.
