# Why EverArcade Is Different

Traditional games center the server/application. EverArcade centers the world.

That shift changes the architecture. A world should be packaged, executed, checked, replayed, restored, and carried forward by evidence rather than by trust in one opaque deployment.

| Model | Center of authority | What users usually trust | EverArcade difference |
| --- | --- | --- | --- |
| Traditional MMOs | Studio server cluster | The operator's database and live service | The world state should be replayable from packages, inputs, checkpoints, and receipts. |
| Centralized game servers | Application process | Admin control and logs | Operators host execution but should not secretly redefine world history. |
| Roblox-like platforms | Platform runtime and marketplace | Platform policy and creator scripts | EverArcade emphasizes deterministic world packages and verifiable continuity. |
| Steam-style distribution | Downloaded application | Build distribution and account systems | Distribution is not enough; world execution should produce replay evidence. |
| Blockchain games | On-chain assets or settlement | Contract state and token ownership | EverArcade keeps game execution in a deterministic runtime and treats ledger anchoring as a boundary. |
| Ordinary smart contracts | Contract execution on chain | Chain consensus | EverArcade targets richer worlds where off-chain deterministic execution can still be replayed and rooted. |

## Architecture focus

EverArcade is not trying to make every frame, asset, or gameplay interaction a ledger transaction. It is trying to make the world lifecycle verifiable: package, execute, receipt, checkpoint, replay, restore, and evolve.

## The world-first principle

A world-first architecture asks:

- Can another operator reproduce this state?
- Can a developer debug the exact mutation window?
- Can players and infrastructure providers distinguish history from an operator claim?
- Can a world survive hosting changes without becoming a new world?
