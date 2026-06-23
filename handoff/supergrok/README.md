# SuperGrok Handoff Bundle — EverArcade Compiler

**Generated:** 2026-06-23  
**Repo:** `https://github.com/lj26ft/everarcade-compiler`  
**Branch:** `main` (clean working tree at generation time)  
**Purpose:** Give a cloud SuperGrok session enough structured context to reason about the entire repository without re-exploring from scratch.

---

## How to use this bundle

1. **Start here:** `00-EXECUTIVE_SUMMARY.md` — what the repo is, what works, what does not.
2. **Architecture:** `01-ARCHITECTURE.md` — authority boundaries, dual stacks, data flow.
3. **Truth vs. appearance:** `02-MATURITY_AND_REALITY.md` — ALPHA vs SCAFFOLD vs misleading PASS reports.
4. **Navigation:** `03-NAVIGATION_INDEX.md` — where to look for any concern.
5. **Open source:** `04-OPEN_SOURCE_READINESS.md` — gaps, priorities, recommended next task.
6. **Commands:** `05-VALIDATION_COMMANDS.md` — canonical scripts and prerequisites.
7. **Gotchas:** `06-GOTCHAS_AND_DUAL_STACKS.md` — integration traps for agents.
8. **Machine indexes:** `MANIFEST.json`, `07-KEY_FILES.json`, `08-REPO_SNAPSHOT.json`

### Single-file upload option

If the cloud session accepts one large context file, concatenate in this order:

```
00-EXECUTIVE_SUMMARY.md
01-ARCHITECTURE.md
02-MATURITY_AND_REALITY.md
03-NAVIGATION_INDEX.md
04-OPEN_SOURCE_READINESS.md
05-VALIDATION_COMMANDS.md
06-GOTCHAS_AND_DUAL_STACKS.md
```

Then attach `MANIFEST.json` and `07-KEY_FILES.json` for lookup.

### Canonical repo docs (read in cloud if needed)

These live **outside** this bundle but supersede it when they conflict:

| File | Role |
|------|------|
| `README.md` | Public entry, supported today path |
| `MATURITY.md` | Subsystem maturity truth table |
| `REPOSITORY_MAP.md` | Directory ownership |
| `OPEN_SOURCE_READINESS.md` | Maintainer audit |
| `docs/03-system-architecture.md` | Platform authority |
| `docs/14-v0.1-architecture-freeze.md` | v0.1 scope freeze |
| `docs/runtime-capabilities.md` | Implemented vs scaffold matrix |
| `docs/12-gap-analysis.md` | Prioritized gaps |

### Agent operating rules

- **Do not** infer production readiness from `*: PASS` reports in `reports/`.
- **Do** treat `MATURITY.md` + targeted validation scripts as authority.
- **Default proof path:** Creator SDK → `dist/runtime-package` → `everarcade-runtime` → receipts/journal/checkpoints → replay verify.
- **Avoid** broad `cargo test --workspace` unless explicitly required; use targeted scripts with `CARGO_BUILD_JOBS=1`.
- **Public frontend** is external (`everarcade-frontend`); in-repo `frontend/` is prototype only.

### Recommended first commands in cloud clone

```bash
cd everarcade-compiler
bash handoff/supergrok/05-VALIDATION_COMMANDS.md  # read first
CARGO_BUILD_JOBS=1 bash scripts/validate_developer_onboarding.sh
bash examples/reference-certified-world-v1/operator/verify.sh examples/reference-certified-world-v1
```

---

## Bundle contents

| File | Description |
|------|-------------|
| `README.md` | This file |
| `MANIFEST.json` | Bundle metadata and reading order |
| `00-EXECUTIVE_SUMMARY.md` | One-page repo state |
| `01-ARCHITECTURE.md` | System design and stacks |
| `02-MATURITY_AND_REALITY.md` | Classification and confusion risks |
| `03-NAVIGATION_INDEX.md` | Directory and concern index |
| `04-OPEN_SOURCE_READINESS.md` | Release gaps and suggested work |
| `05-VALIDATION_COMMANDS.md` | Scripts, prerequisites, gates |
| `06-GOTCHAS_AND_DUAL_STACKS.md` | Traps for automated reasoning |
| `07-KEY_FILES.json` | ~80 critical paths with roles |
| `08-REPO_SNAPSHOT.json` | Metrics, git, workspace, warnings |

## Packaging

```bash
tar -czf handoff/supergrok-handoff-v1.tar.gz -C handoff supergrok
```