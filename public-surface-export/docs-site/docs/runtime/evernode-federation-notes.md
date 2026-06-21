# Evernode Federation Notes

## Measured
- Deterministic fixture-driven federation validation succeeds for 2/3/5-node, partitioned, restored, and archive recovery scenarios.

## Inferred
- Replay retention and archive growth pressure storage as eras increase.
- Bandwidth pressure tracks archive synchronization frequency and checkpoint reconstruction.

## Speculative
- Unknown scaling limits likely emerge from very large archive eras and frequent restoration churn.
- Operational risk increases when topology churn outpaces deterministic restoration windows.
