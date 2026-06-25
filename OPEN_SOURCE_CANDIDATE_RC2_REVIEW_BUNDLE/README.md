# Open Source Candidate RC2 Review Bundle

This bundle is a self-contained reviewer map for reproducing and verifying Open Source Candidate RC2 from a fresh environment.

## Contents

- `TRUST_ROOT.md` — committed review trust root and pinned key.
- `PAYLOAD_BINDING.md` — explicit transitive hash binding from `world.evr` to payload files.
- `REVIEW_TRUST_CHAIN.md` — trust begins at the official trust root and flows through attestation, artifact, payload, replay, and runtime.
- `GATE_PROOFS.md` — what each gate proves and does not prove.
- `fixtures/self-attested-fork/` — must-fail fixture for a fork that signs with its own generated key.
- `fixtures/tampered-reference-world/` — must-fail fixture for payload tampering without hash updates.
- `reference-world/must-pass/` — expected canonical world behavior.
- `reference-world/must-fail/` — expected negative behavior.
- `expected-outputs/` — expected command outcomes.

## Fresh Reviewer Workflow

```bash
git clone <repository-url> everarcade-compiler
cd everarcade-compiler
git checkout b6d553d5632a88b95352d6c98d15503b27d4df0f
bash scripts/ensure_vendor_offline.sh
bash scripts/check_prerequisites.sh
bash scripts/validate_open_source_readiness.sh
EVERARCADE_DETERMINISTIC_ATTEST=1 CARGO_BUILD_JOBS=1 bash scripts/ci/run-deterministic-world-factory.sh
TRUSTED_PUBLIC_KEY="$(awk '/^```text$/{block++; next} block==1 && /^[A-Za-z0-9+\/=]+$/{print; exit}' TRUST_ROOT.md)"
node creator-sdk/cli/everarcade.mjs world attest verify \
  --project examples/world-factory/frontier-settlement \
  --trusted-public-key "$TRUSTED_PUBLIC_KEY"
node specs/world-evr-package/verify-package-v1.mjs \
  examples/world-factory/frontier-settlement/out/world.evr
```

Then execute both must-fail fixture procedures. Each negative fixture must fail before RC2 can pass review.
