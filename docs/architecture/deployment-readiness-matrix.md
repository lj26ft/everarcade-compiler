# EverArcade Deployment Readiness Matrix

Canonical assessment date: 2026-06-03

| Subsystem | Status | Readiness % | Blocking Issues | Priority |
| --- | --- | ---: | --- | --- |
| Deterministic Execution Core | Deployment Ready | 86 | Canonical verifier CLI; artifact-only replay/checkpoint proof; stable public API facade | Critical |
| State Persistence | Experimental | 72 | Durable crash consistency; restore rehearsal gate; backup/archive policy | Critical |
| Federation Runtime | Scaffold | 45 | Cross-machine multi-node proof; peer auth/rate limits; split-brain recovery; operator topology controls | High |
| World Runtime | Experimental | 62 | Stateful host loop; session/input integration; continuous-operation soak; restart recovery | Critical |
| Renderer Runtime | Scaffold | 38 | Renderer-client tests; production projection stream; missed-frame catch-up; observer recovery | Medium |
| Deployment Infrastructure | Experimental | 54 | Clean-VM artifact workflow; service lifecycle; upgrade/rollback gates; placeholder script labeling | Critical |
| Evernode Readiness | Scaffold | 42 | Live lease deployment; provider lifecycle commands; lease storage/recovery/resource validation | Critical |
| XRPL/Xahau Integration | Scaffold | 24 | Live adapter, key management, idempotent settlement queue, ledger verification/finality | Medium |
| Developer Experience | Experimental | 46 | Canonical template/quickstart; CLI integration tests; stable SDK facade; docs consolidation | High |
| Operational Readiness | Scaffold | 34 | Metrics/log/alert pipeline; backup automation; incident runbooks; disaster recovery drills | Critical |

## Matrix notes

- The execution core is the strongest area because deterministic WASM execution, receipts, roots, checkpoints, and extensive tests exist.
- Persistence is functional but not yet production-durable; local JSON records and integrity checks must be hardened into a crash-safe store.
- Federation, renderer/history, and Evernode are not scored as production runtime domains despite broad architecture coverage because live recoverable behavior is not proven.
- XRPL/Xahau is intentionally low-scored because implemented code is primarily deterministic payload/intent construction plus dry-run or placeholder submission.
- Developer experience is suitable for internal/developer-preview use, not self-service external onboarding.
- Operations is the largest commercial blocker after durable runtime hosting because monitoring, alerting, backup, incident response, and disaster recovery are not wired end to end.

## Deployment gate recommendation

A v0.1 release candidate should not be promoted until the following matrix rows reach at least these minimums:

| Subsystem | Minimum for v0.1 | Required classification |
| --- | ---: | --- |
| Deterministic Execution Core | 90 | Deployment Ready |
| State Persistence | 85 | Deployment Ready |
| World Runtime | 80 | Deployment Ready |
| Deployment Infrastructure | 80 | Deployment Ready |
| Evernode Readiness | 75 | Experimental or Deployment Ready with live evidence |
| Operational Readiness | 60 | Experimental with concrete runbooks/gates |
| Developer Experience | 65 | Experimental with tested quickstart |

Federation, renderer, and XRPL/Xahau can remain below those thresholds only if the v0.1 release promise explicitly excludes production federation, production renderer/history service, and live settlement.
