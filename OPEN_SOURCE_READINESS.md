# Open Source Readiness

EverArcade enforces two protected CI gates so contributors reproduce the same environment and world artifacts on every machine:

```text
Contributor Gate          →  Can a contributor reproduce EverArcade?
World Artifact Gate       →  Can this repository still produce a valid world artifact?
```

Together they establish:

```text
fresh clone
↓
reproducible environment
↓
deterministic world generation
↓
verification
↓
attestation
↓
release packaging
```

Trust the process, not the machine.

---

## Open source readiness milestone

EverArcade now guarantees:

| Capability | Status |
|------------|--------|
| Contributor Gate | ✓ |
| World Artifact Gate | ✓ |
| Canonical vendor | ✓ |
| Offline build recovery | ✓ |
| Deterministic generation | ✓ |
| Verification pipeline | ✓ |
| Attestation pipeline | ✓ |
| Release packaging | ✓ |
| GitHub CI enforcement | ✓ |

This marks the transition from a single-machine project to **reproducible open-source infrastructure**.

---

## Contributor Gate

**Purpose:** Can a contributor reproduce EverArcade?

The Contributor Gate verifies:

- canonical vendor restoration
- offline dependency recovery
- toolchain compatibility
- repository readiness
- deterministic environment setup

**GitHub Actions workflow:** [`.github/workflows/onboarding.yml`](.github/workflows/onboarding.yml)

**Local equivalent:**

```bash
bash scripts/ensure_vendor_offline.sh
bash scripts/validate_open_source_readiness.sh
```

**Expected result:** `READY` (classification in `reports/open_source_readiness_report.txt`)

A contributor who passes this gate has successfully recreated the EverArcade development environment.

The workflow also runs developer onboarding validation and reference-world verification on Ubuntu and macOS. See [`CONTRIBUTING.md`](CONTRIBUTING.md) for the full 3-command onboarding gate.

---

## World Artifact Gate

**Purpose:** Can this repository still produce a valid EverArcade world artifact?

The World Artifact Gate verifies:

- world generation
- package verification
- runtime execution
- replay verification
- deployment bundle creation
- release attestation
- release packaging
- deterministic outputs

**GitHub Actions workflow:** [`.github/workflows/deterministic-world-factory.yml`](.github/workflows/deterministic-world-factory.yml)

**Local equivalent:**

```bash
bash scripts/ci/run-deterministic-world-factory.sh
```

**Expected result:** `Deterministic World Factory CI: PASS`

A contributor who passes this gate has successfully reproduced the complete EverArcade world pipeline.

---

## Contributor Level 0

A new contributor reaches **Level 0** when the following complete without modifying repository code:

```bash
git clone <repo>
cd everarcade-compiler

bash scripts/ensure_vendor_offline.sh
bash scripts/validate_open_source_readiness.sh
bash scripts/ci/run-deterministic-world-factory.sh
```

**Expected outputs:**

```text
READY
PASS
```

Level 0 means:

```text
environment reproduced
world pipeline reproduced
verification reproduced
attestation reproduced
```

before any code changes are made.

---

## World artifact workflow

The canonical artifact path:

```text
Blueprint
↓
Contract Plan
↓
world.evr
↓
Verification
↓
Runtime
↓
Replay
↓
Deployment
↓
Attestation
↓
Release Bundle
```

**Commands** (default project: `examples/world-factory/frontier-settlement`):

```bash
node creator-sdk/cli/everarcade.mjs world factory generate

node creator-sdk/cli/everarcade.mjs world factory verify

node creator-sdk/cli/everarcade.mjs world factory boot

node creator-sdk/cli/everarcade.mjs world factory run --ticks 100

node creator-sdk/cli/everarcade.mjs world factory replay

node creator-sdk/cli/everarcade.mjs world factory deploy

node creator-sdk/cli/everarcade.mjs world attest create

node creator-sdk/cli/everarcade.mjs world attest verify \
  --trusted-public-key "$(awk '/^```text$/{block++; next} block==1 && /^[A-Za-z0-9+\/=]+$/{print; exit}' TRUST_ROOT.md)"

node creator-sdk/cli/everarcade.mjs release build \
  --project examples/world-factory/frontier-settlement
```

See [`docs/reproducible-builds.md`](docs/reproducible-builds.md) for vendor restore, determinism checks, and known non-deterministic fields.

---

## CI enforcement

Every pull request must preserve both gates. GitHub Actions verifies:

- reproducible environment
- deterministic world generation
- package verification
- replay verification
- attestation verification
- release bundle integrity
- vendor integrity

A pull request that breaks either gate should fail CI.

| Gate | Workflow | Primary failure modes |
|------|----------|----------------------|
| Contributor Gate | `onboarding.yml` | missing vendor artifact, offline cargo failure, onboarding script failure |
| World Artifact Gate | `deterministic-world-factory.yml` | fmt drift, world factory pipeline failure, determinism mismatch, release bundle over 100 MB |

---

## Contributor invitation template

Before receiving contributor access, a contributor should run:

```bash
bash scripts/ensure_vendor_offline.sh
bash scripts/validate_open_source_readiness.sh
bash scripts/ci/run-deterministic-world-factory.sh
```

and achieve:

```text
READY
PASS
```

without modifying repository code. This ensures every contributor begins from the same verified baseline.

---

## Strategic meaning

**Contributor Gate** protects:

```text
reproducibility
environment integrity
onboarding reliability
```

**World Artifact Gate** protects:

```text
world generation
verification
attestation
release integrity
```

---

## Honest scope boundaries

EverArcade v0.1 is an open-source **candidate** focused on local deterministic runtime proofs. Passing both gates does **not** claim production, public-testnet, commercial, XRPL/Xaman, GPU marketplace, or federation readiness. See [`MATURITY.md`](MATURITY.md) and [`README.md`](README.md).

**Alpha areas:** execution core, runtime local proof path, Creator SDK, world packages, RustRigs.

**Scaffold or experimental areas:** renderer/history/federation, XRPL settlement, Xahau hooks, GPU marketplace, developer portal, player gateway, commercial revenue, public testnet.

**Release risks to watch:**

1. Users may confuse local PASS reports with production readiness.
2. Settlement and marketplace directories may imply live production capabilities.
3. Federation and renderer docs may be mistaken for canonical runtime behavior.
4. Vendor artifact drift may break offline builds — regenerate with `bash scripts/vendor_deps.sh` and commit updated `dist/vendor.tar.gz` + `vendor.sha256`.
## Open Source Candidate RC1 clarification

The RC1 review separates two paths that were previously easy to conflate:

| Path | Produces | Does not produce |
|------|----------|------------------|
| Contributor Gate | pinned toolchain check, restored vendor tree, local runtime package checks, onboarding/readiness reports | certified `world.evr`, release attestation, deployment bundle |
| World Artifact Gate | Frontier Settlement `world.evr`, package verification, replay verification, attestation verification, deployment/release bundle | a generalized production/public-testnet guarantee |

Current reproducibility guarantee: EverArcade is reproducible for the pinned Rust version in `rust-toolchain.toml`, pinned Node version in `.nvmrc`, committed vendor bundle, and isolated CI gate commands. The project does not claim arbitrary-toolchain or arbitrary-machine determinism beyond those pins.
