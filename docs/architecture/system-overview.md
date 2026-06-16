# System Overview

EverArcade separates world rules, runtime execution, operator infrastructure, player clients, and verification artifacts.

```mermaid
flowchart LR
  Dev[Developer] --> Contract[World Contract]
  Contract --> Package[World Package]
  Package --> Operator[Operator Runtime]
  Player[Player Client] --> Operator
  Operator --> Receipts[Receipts and Proof Bundles]
  Operator --> Checkpoints[Checkpoints]
  Receipts --> Verifier[Independent Verification]
  Checkpoints --> Restore[Restore and Continue]
```

For deeper background, see the [original system architecture](./system-architecture-overview) and [runtime boundaries](./runtime-boundaries).
