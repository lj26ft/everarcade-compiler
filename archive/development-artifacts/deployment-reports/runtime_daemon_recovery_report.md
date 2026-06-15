# Runtime Daemon Recovery Report

- deterministic_daemon_start: `RuntimeDaemonRecoveryRuntime::start` derives a stable genesis continuity root per node id.
- deterministic_daemon_restart: `RuntimeDaemonRecoveryRuntime::restart` restores only from the daemon checkpoint root.
- checkpoint_recovery: daemon checkpoints bind node id, replay height, and continuity root.
- health_gated_readiness: readiness requires running, healthy, and non-authoritative state.
- renderer_authority: renderer/daemon path remains non-authoritative.
- validation_tests: `test_runtime_daemon_startup`, `test_runtime_daemon_restart_recovery`, `test_runtime_daemon_health_gate`, `test_runtime_daemon_non_authoritative`.
