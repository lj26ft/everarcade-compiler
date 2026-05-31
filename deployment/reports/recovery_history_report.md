# Recovery History Report

## evidence
- Two-node harness records modeled failures, recoveries, restorations, and rejoins in deterministic tests.

## test coverage
- failures: `test_node_failure_survival`
- recoveries: `test_node_recovery_convergence`
- restorations: checkpoint restoration in `test_checkpoint_convergence`
- rejoins: `test_node_join_certification`

## known limitations
- History is certification-harness evidence, not production daemon audit logs.

## remaining scaffolds
- Durable operator recovery journal.
- Cross-host recovery timeline capture.

## next risks
- Disk corruption and interrupted restore should be tested before deployment claims.
