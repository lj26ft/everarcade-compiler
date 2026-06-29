# world-evr-verify Commitment V1 Follow-up

The independent `world-evr-verify` repository must implement verification for:

- `docs/specs/COMMITMENT_ARCHITECTURE_V1.md`;
- `docs/specs/RECEIPT_MMR_V1.md`;
- `docs/specs/CHECKPOINT_V1.md`;
- `docs/specs/PROOF_FORMAT_V1.md`;
- `test-vectors/commitments/receipt-mmr-v1.json`.

The verifier must answer:

- Is this `world.evr` package valid?
- Is this receipt included in this world history?
- Does this checkpoint commit to the claimed state, receipt, and continuity roots?
- Does this `world_hash` rederive correctly?
- Is this proof valid under `world.evr.commitment.v1`?

It must reject malformed hex, unknown versions, unsupported commitment profiles, legacy proofs under v1, v1 proofs under legacy mode, duplicated or missing fields, mismatched receipt/world/tick/sequence data, checkpoint mismatches, wrong domain tags, wrong peak ordering, and wrong empty roots.
