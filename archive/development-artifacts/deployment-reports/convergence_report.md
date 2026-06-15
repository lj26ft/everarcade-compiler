# Convergence Report

## evidence
- Deterministic two-node convergence tests were executed by this script.
- Metrics recorded by the harness include sync ticks, checkpoint transfers, replay transfers, recovery ticks, and convergence ticks.

## test coverage
- world root convergence
- replay root/hash convergence
- checkpoint root convergence
- continuity root convergence

## known limitations
- Metrics are informational and measured in deterministic harness ticks, not wall-clock distributed network time.

## remaining scaffolds
- Cross-host transport timings.
- Operator-level convergence dashboards.

## next risks
- Long-lived storage and real transport jitter can still expose non-harness convergence risks.
