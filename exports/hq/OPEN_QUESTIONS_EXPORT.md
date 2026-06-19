# EverArcade Compiler Open Questions Export

Export date: 2026-06-19

1. How much of v0.1 should be open source at launch?
   - Need final boundary between public developer-preview code/docs and any temporarily withheld operational material.

2. What is the operator launch sequence?
   - Need rehearsal order for package verification, runtime start, status checks, replay verification, restore evidence, and demo/operator handoff.

3. What is the public demo hosting method?
   - Options include local recording only, static hosted projection, VPS wrapper route, or a constrained public demo endpoint.

4. What is the grant strategy?
   - Need narrative package grounded in implemented evidence: World Package, canonicalizer/GAP-2, RustRig proofs, Projection Runtime, Arena Vanguard, and architecture freeze.

5. Where is the Hooks/XRPL/Xahau v0.2 boundary?
   - v0.1 must not claim live settlement; v0.2 needs a clean boundary for Hooks, XRPL/Xahau anchoring, wallet/vault, and settlement claims.

6. Which RustRig does HugeGreenCandle prove first?
   - Candidate choices should be selected from the RustRig Standard Library candidate set and/or the existing certified set.

7. How should canonicalizer integration enter the playable demo root path?
   - GAP-2 is closed in the canonicalizer, but the exact integration point for the playable demo root path needs explicit follow-up.

8. What vendored Cargo dependency gaps remain?
   - Review vendored/offline dependency coverage before broad open-source contribution and before promising reproducible external builds.
