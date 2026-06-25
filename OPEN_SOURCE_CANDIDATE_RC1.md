# Open Source Candidate RC1

## Resolved findings

- F1: `manifest_sha256` is now specified as the SHA-256 digest of the exact `world.evr/manifest.json` bytes, using raw lowercase hex inputs with no `sha256:` prefix in the V0.1 RC2 derivation recipe.
- F2: Toolchains are pinned by `rust-toolchain.toml` and `.nvmrc`; `scripts/check_prerequisites.sh` validates `rustc`, `cargo`, `node`, `npm`, `git`, `tar`, and a SHA-256 tool before gates run.
- F3: Contributor Gate and World Artifact Gate outputs are explicitly separated in contributor, readiness, repository map, and onboarding docs.
- F4: `open-source-candidate/` and `OPEN_SOURCE_CANDIDATE_RC1_REVIEW_BUNDLE/` provide Frontier Settlement RC1 reference inputs, workflows, gate scripts, instructions, and expected PASS outputs.
- F5: Determinism claims are scoped to pinned toolchain, pinned vendor, and isolated CI/local gate runs.

## Remaining risks

- Cross-machine reproducibility outside the pinned Rust/Node/vendor tuple is not claimed.
- Public-testnet, production, GPU marketplace, renderer/history/federation, and commercial readiness remain out of scope for RC1.

## Reproducibility guarantee

A clean reviewer using the pinned Rust version, pinned Node version, committed vendor bundle, and documented gates can clone, restore vendor dependencies, run the Contributor Gate, run the World Artifact Gate, verify attestation, and obtain `PASS` without maintainer-only context.

## Review process

```bash
git clone <repo>
cd everarcade-compiler
bash scripts/check_prerequisites.sh
bash scripts/ensure_vendor_offline.sh
bash scripts/validate_open_source_readiness.sh
EVERARCADE_DETERMINISTIC_ATTEST=1 bash scripts/ci/run-deterministic-world-factory.sh
node creator-sdk/cli/everarcade.mjs world attest verify \
  --project examples/world-factory/frontier-settlement \
  --trusted-public-key "$(awk '/^```text$/{block++; next} block==1 && /^[A-Za-z0-9+\/=]+$/{print; exit}' TRUST_ROOT.md)"
tar -czf everarcade-open-source-candidate-rc1.tar.gz OPEN_SOURCE_CANDIDATE_RC1.md OPEN_SOURCE_CANDIDATE_RC1_REVIEW_BUNDLE open-source-candidate
```
