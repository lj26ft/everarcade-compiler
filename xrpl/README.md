# XRPL Live Settlement Layer v0.1

This directory contains deterministic XRPL-aware settlement artifacts for the EverArcade runtime boundary.

## Layout

- `intent/` stores replay-safe settlement intents and their intent roots.
- `transactions/` stores unsigned XRPL Payment transaction representations derived from intents.
- `receipts/` stores deterministic XRPL receipt evidence and imported settlement evidence.
- `anchors/` stores continuity anchors binding EverArcade continuity to settlement evidence.
- `verification/` stores transaction verification transcripts and replay validation records.
- `live_settlement_model.sh` defines the deterministic model used by validation and certification scripts.

The layer models transaction representation and receipt integration only. It does not sign, submit, federate, or alter XRPL consensus.
