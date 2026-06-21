# Compiler Export — 2026-06-21

This export summarizes current `everarcade-compiler` reality for HQ, docs, and public messaging.

## What exists

- **Replay proofs:** independent third-party oracle reproduced replay end-to-end over 5 production journals.
- **Commitment verification:** replay verifier recomputes state, receipt, world, and continuity commitments from the published journal fixtures.
- **Restore proofs:** restore verifier and fixtures exist; receipt roots are derived from `receipt_accumulator`, not trusted from stored roots.
- **Migration proofs:** migration verifier and fixture exist; destination continuity is linked to source export and continuation journal.
- **World Registry:** CLI/status surface is implemented and ready to feed public world discovery.
- **Capability Marketplace:** model/API surface is implemented; validation is blocked by a vendored dependency issue.
- **Treasury Execution Layer:** model/helpers are implemented; validation is blocked by a vendored dependency issue.
- **Vision:** `/vision` docs page exists and leads with `world.evr`.
- **Founding Worlds:** outreach/application/manual language exists in docs and portal copy.
- **Docs build:** docs are Markdown-only in this repo; public docs pages now center verified world infrastructure.
- **World portal:** `frontend/world-portal` renders static registry fixtures for `/worlds`, `/worlds/:worldId`, `/founding-worlds`, and `/capabilities`.

## What is independently verified

- Replay was independently reproduced over 5 production journals.
- The current restore and migration oracle scripts recompute receipt roots from `final_receipt_hashes + last_temp_receipt_hash` accumulators.

## What is fixture-witnessed

- Restore is reproducible over supplied fixtures with bundle data and receipt accumulator evidence.
- Migration is reproducible over supplied fixtures with export bundle, restore bundle, migration link, and continuation journal evidence.

## Implemented but not fully tested

- Capability Marketplace model/API: implemented; test blocked by vendored dependency issue.
- Treasury Execution Layer model/helpers: implemented; test blocked by vendored dependency issue.

## Known test blockers

- Vendored dependency issue blocks marketplace and treasury validation tests.
- Do not claim formal verification for all inputs; claim only independent replay reproduction and fixture-witnessed restore/migration reproduction.

## Untracked files intentionally ignored

- `runtime/gpu-marketplace/Cargo.lock` is untracked and intentionally left untouched by this milestone.

## Next

1. Re-run independent restore/migration verification after the accumulator fix is reviewed externally.
2. Replace static world portal fixtures with a live registry API.
3. Publish Founding Worlds application flow publicly.
4. Convert capability and treasury blocked tests once the vendored dependency issue is resolved.
