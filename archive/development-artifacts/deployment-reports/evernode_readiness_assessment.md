# EverNode Readiness Assessment

## evidence
- Single-crate deterministic two-node runtime certification has a local harness and targeted tests.

## test coverage
- single node readiness: partially implemented for deterministic boot/checkpoint/replay.
- two node readiness: implemented locally for join, convergence, failure, recovery, divergence, and authority preservation.
- deployment readiness: scaffold only.
- operator readiness: scaffold only.

## known limitations
- This is not production readiness and does not include live EverNode deployment.

## remaining scaffolds
- Operator packaging.
- Host-level monitoring.
- Network and storage fault injection.

## next risks
- EverNode deployment should remain blocked until cross-machine recovery evidence exists.
