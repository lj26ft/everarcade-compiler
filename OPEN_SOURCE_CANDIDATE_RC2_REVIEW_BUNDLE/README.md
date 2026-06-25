# Open Source Candidate RC2 Review Bundle

This bundle is a self-contained reviewer map for reproducing and verifying Open Source Candidate RC2 from a fresh environment.

## Contents

- `TRUST_ROOT.md` — committed review trust root and pinned key. CI/review tests use the matching TEST-ONLY fixture in `fixtures/trust-root/`.
- `PAYLOAD_BINDING.md` — explicit transitive hash binding from `world.evr` to payload files.
- `REVIEW_TRUST_CHAIN.md` — trust begins at the official trust root and flows through attestation, artifact, payload, replay, and runtime.
- `GATE_PROOFS.md` — what each gate proves and does not prove.
- `fixtures/self-attested-fork/` — must-fail fixture for a fork that signs with its own generated key.
- `fixtures/tampered-reference-world/` — must-fail fixture for payload tampering without hash updates.
- `reference-world/must-pass/` — expected canonical world behavior.
- `reference-world/must-fail/` — expected negative behavior.
- `expected-outputs/` — expected command outcomes.


## Authoritative RC2 Pins

RC2 distinguishes the original artifact/export commit from the runnable reviewer-entry commit:

- RC2 artifact/export commit: `fe51c1ce5be6df888dfaae203d5632580a045f2e`
- RC2 reviewer-entry commit: `53a17567e826c5d4f9b083e490cf1568bfe7534e`

Reviewers should check out the reviewer-entry commit before running the gate or must-fail fixtures. The artifact/export commit remains the historical RC2 artifact reference, but it predates the reviewer wrapper scripts.

## RC2 Independent Reviewer Path

Use this single path to reproduce the authoritative RC2 review target from a fresh clone:

```bash
git clone git@github.com:EverArcade/everarcade-compiler.git
cd everarcade-compiler
git checkout 53a17567e826c5d4f9b083e490cf1568bfe7534e

scripts/ci/check-rc2-commit-pins.sh

# Run the normal RC2 gate.
CARGO_BUILD_JOBS=1 scripts/ci/rc2-gate.sh

# Run must-fail fixture: self-attested fork must fail.
scripts/ci/rc2-fixture-self-attested-fork-must-fail.sh

# Run must-fail fixture: tampered payload must fail.
scripts/ci/rc2-fixture-tampered-payload-must-fail.sh
```

Expected result:

- The normal RC2 gate passes.
- The commit-pin consistency check passes.
- The self-attested-fork fixture fails verification.
- The tampered-payload fixture fails verification.
- If either must-fail fixture passes, RC2 is not valid.
