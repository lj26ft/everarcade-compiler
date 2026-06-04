# GPU Runtime v0.1

`runtime/gpu-runtime/` is the operational runtime scaffold that bridges the Renderer Runtime to the deterministic GPU model in `gpu/`.

It represents three read-only stages:

1. Projection export from the Renderer Runtime.
2. GPU job submission into deterministic queues.
3. Render artifact import back into renderer-visible output roots.

The runtime is non-authoritative and must never mutate protocol state.
