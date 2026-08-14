# Task 4D.2 — Frozen State V10 compiler artifact

The compiler deterministically rebuilds the frozen State V10 Runtime IR (`sha256:1cbfe2e08ea0e724076b38e5ddb5cf21b9f5b998f6ffc04c3260c6abcd230111`) and `world.evr` (`sha256:8f859c32508c864a7129845297f8cf91559f40ed43710e34bde89a14ddde8348`). The package binds Treasury v8, semantic policy v2, the 23-action inventory, receipt/action mapping, checkpoint v2, and State V9-to-V10 migration policy.

Two repository-wide hygiene exceptions are outside this artifact boundary: pre-existing `cargo fmt --check` drift and three Arena Vanguard tests whose deployment report fixtures are absent. Targeted deterministic package reconstruction and State V2–V9 compatibility pass; neither exception changes the frozen package bytes.
