# Crash Recovery Model

## Measured
- Crash scenarios validate deterministic accept/reject outcomes.

## Inferred
- Replay reconstruction and scheduler restart recovery preserve continuity when restoration manifests are valid.

## Speculative
- Durable storage interruptions beyond current test vectors may need additional manifests.
