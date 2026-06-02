# Player Sessions

Checkpoint-safe and replay-safe persistence root for Arena Vanguard hosted sessions.

Persisted fields: PlayerId, CharacterId, Inventory, Level, XP, Position, and resume token. Runtime host writes this data during checkpoints and disconnect; gateway only reads status/resume routing metadata.
