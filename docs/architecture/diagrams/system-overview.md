# System Overview Diagram

```mermaid
flowchart LR
  Developer --> WorldContract[World Contract]
  WorldContract --> WorldPackage[World Package]
  WorldPackage --> Runtime[Operator Runtime]
  Player --> Client[Player Client]
  Client --> Runtime
  Runtime --> History[World History]
  Runtime --> Checkpoints
  History --> Verifier[Independent Verifier]
  Checkpoints --> Recovery[Restore and Continue]
```
