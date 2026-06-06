# Open Source Readiness Review

**Date:** 2026-06-05  
**Final result:** **NOT READY**

## Executive summary

The repository is not ready for public open-source launch. It contains useful prototype code and extensive documentation, but public readers could confuse scaffold/model artifacts with live platform features. Basic open-source governance files are missing or unclear, dependency policy needs cleanup, and security boundaries need stronger labels.

## Checklist

| Area | Status | Finding |
| --- | --- | --- |
| license | BLOCKER | No root `LICENSE` file was found. Public reuse terms are unclear. |
| secrets | WARNING | Secret scan found mostly false positives and docs, but no formal secret scanning gate is documented. |
| keys | WARNING | XRPL/Xaman docs correctly say keys are not stored, but public users need a clear no-keys policy and examples must avoid real credentials. |
| credentials | WARNING | No obvious production credentials found in sampled scan, but committed records should be reviewed before public release. |
| large binaries | PASS | No files larger than 5 MB were found outside `.git`, `vendor`, and `node_modules` in the sampled check. |
| generated artifacts | BLOCKER | Many `.records`, reports, root marker files, status files, and `node_modules/` are present. Policy is unclear. |
| vendor policy | BLOCKER | `.cargo/config.toml` forces offline vendored dependencies, but targeted runtime build failed because vendor/ lacked `bincode`. |
| security warnings | BLOCKER | Abuse analysis shows settlement, wallet, marketplace, GPU-provider, replay, checkpoint, and template risks need prominent warnings. |
| unclear claims | BLOCKER | PASS reports and readiness docs can overstate production reality. |
| private business docs | WARNING | Commercial revenue and marketplace planning records should be reviewed for business-sensitive assumptions. |
| unsafe scripts | WARNING | Hundreds of shell scripts exist; some may be safe models, but public support boundaries are unclear. |
| missing contributing guide | BLOCKER | No root `CONTRIBUTING.md` found. |
| missing security policy | BLOCKER | No root `SECURITY.md` found. |
| missing code of conduct | BLOCKER | No root `CODE_OF_CONDUCT.md` found. |
| missing architecture map | IMPROVED | This audit added `docs/architecture/human-readable-repo-map.md`, but it should be adopted as canonical and maintained. |

## Required before public release

1. Add root `LICENSE`, `SECURITY.md`, `CONTRIBUTING.md`, and `CODE_OF_CONDUCT.md`.
2. Fix vendored dependency completeness or remove forced offline mode for public contributors.
3. Remove, ignore, or document `node_modules/` and generated artifacts.
4. Mark all scaffold/model/PASS reports with execution class: live, simulated, dry-run, script-only, documentation-only, or certification model.
5. Add a supported command matrix: what a new user can run, what it proves, and what it does not prove.
6. Add a public threat model and vulnerability disclosure process.
7. Review commercial/revenue/marketplace docs for sensitive business claims.
8. Add CI checks for secrets, generated artifacts, formatting, and the reality-audit required files.

## Final result

**NOT READY**

The repo can be open-sourced later as a pre-production prototype if claims are narrowed and the blockers above are resolved.
