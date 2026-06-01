# Executable Rustrigs Report

All required primitive modules expose concrete deterministic functions that return `Vec<ProtocolRecord>` and avoid IO, authority writes, XRPL submission, and deployment execution.

| Rustrig Module | Classification | Notes |
| --- | --- | --- |
| combat | Executable | Damage, healing, status, cooldown records. |
| inventory | Executable | Add, remove, transfer, equip, stack, split records. |
| quests | Executable | Start, objective, completion, failure, reward records. |
| dialogue | Executable | Start, choice, node, completion records. |
| economy | Executable | Ledger, transfer, mint, burn, settlement intent records. |
| world | Executable | Spawn, despawn, move, faction, ownership records. |
| progression | Executable | Experience and milestone records. |
| crafting | Executable | Recipe, consume, produce, craft records. |
| factions | Executable | Faction creation, membership, reputation, relation records. |
| movement | Executable | Bounds, movement, teleport records. |
| interaction | Executable | Interaction, trigger, container, object records. |
| xrpl | Executable | Intent-only records; never submits. |
| deployment | Executable | Intent-only records; never deploys. |
