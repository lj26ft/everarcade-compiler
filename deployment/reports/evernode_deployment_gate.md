# EverNode Deployment Gate

| Gate | Result | Reason |
| --- | --- | --- |
| Single node | PASS | Deterministic runtime certification exists. |
| Two node | PASS | Two-node certification harness covers convergence and recovery semantics. |
| Cross machine | FAIL | Harness exists, but physical independent-machine execution is not automated in this repository. |
| Transport | PASS | Actual TCP payload transfer is used by the cross-machine harness. |
| Recovery | PASS | Machine failure and recovery flows are covered by certification tests. |
| Operator workflow | FAIL | Runbooks exist, but production operator automation and host provisioning are not complete. |

## Decision

FAIL for EverNode deployment. The repository now has a certification harness and operator documentation, but production deployment remains blocked until physical host automation and operator workflow validation are completed.
