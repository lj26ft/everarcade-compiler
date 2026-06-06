# Xaman Runtime Layout v0.1

The `xaman/` tree contains deterministic signing-boundary artifacts for the
EverArcade Runtime. It models Xaman payload creation, deep-link and QR metadata,
state tracking, signed receipt import, and replay evidence without storing keys,
signing transactions, or calling live APIs.

## Layout

- `payloads/` - signable settlement payload representations and payload roots.
- `requests/` - deep-link request metadata and QR metadata roots.
- `receipts/` - signed receipt import records and settlement continuity updates.
- `status/` - current payload status summaries and replay validation records.
- `tracking/` - full authorization-state tracking transcripts.
- `signing_model.sh` - deterministic model used by validation and certification scripts.

## Pseudocode authority boundary

- **Input:** signable payload descriptions, request metadata, status tracking records, and imported signed receipt evidence.
- **Output:** deterministic payload IDs, request roots, status records, and receipt-continuity evidence.
- **Authority:** Xaman/user wallet approval is external; this tree must not be treated as a signer or key store.
- **EverArcade fit:** a deterministic wallet UX boundary model that future live integrations can validate against.
