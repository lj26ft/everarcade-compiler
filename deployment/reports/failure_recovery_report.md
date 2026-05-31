# Failure Recovery Report

## evidence
- Node failure survival and recovery convergence tests executed through the two-node harness.
- Surviving node continues deterministic world and replay progression; failed node restores from survivor checkpoint.

## test coverage
- node termination
- survivor continuation
- checkpoint restoration
- replay/state convergence after restore

## known limitations
- Process failure is modeled in memory; no daemon supervisor, disk crash, or multi-host storage fault is claimed here.

## remaining scaffolds
- Operator failover automation.
- Persistent crash-recovery replay store validation.

## next risks
- Real deployment restart behavior and stale checkpoint eviction policies need separate validation.
