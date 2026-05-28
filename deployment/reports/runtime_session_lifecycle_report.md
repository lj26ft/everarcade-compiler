# Runtime Session Lifecycle Report

Replay sessions now exercise deterministic create, checkpoint, restore, interrupted recovery, and equivalence validation paths.

- create_replay_session: binds session id and replay root into a deterministic continuity root.
- checkpoint_session: binds session id, replay root, continuity root, and checkpoint index into a checkpoint root.
- restore_session: rejects checkpoint root mismatches before restoring session state.
- recover_interrupted_session: resumes from the same verified checkpoint path used by restore.
- replay_equivalence: compares session id, replay root, continuity root, and checkpoint index after restoration.
- validation_tests: `test_runtime_session_create_restore`, `test_runtime_session_checkpoint_restore`, `test_runtime_session_recovery_equivalence`, `test_runtime_session_corruption_rejection`.
