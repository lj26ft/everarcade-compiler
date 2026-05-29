# Deterministic Hot Reload

Hot reload reloads deterministic assets from checkpoint-backed runtime state. The workflow plans reloads, preserves replay continuity, restores checkpoints, and rejects invalid authority mutation.

If a reload would create replay divergence, validation fails and the checkpoint remains the recovery source.
