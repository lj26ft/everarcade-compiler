# Quick Start

The absolute minimum local path is: clone, install/check prerequisites, run onboarding, verify.

## 1. Clone

```bash
git clone https://github.com/lj26ft/everarcade-compiler.git
cd everarcade-compiler
```

## 2. Install / check prerequisites

```bash
bash scripts/check_prerequisites.sh
```

## 3. Run

```bash
CARGO_BUILD_JOBS=1 bash scripts/validate_developer_onboarding.sh
```

## 4. Verify

```bash
bash examples/reference-certified-world-v1/operator/verify.sh examples/reference-certified-world-v1
```

Expected result: prerequisite checks pass, onboarding passes, and the reference certified world reports `PASS`.
