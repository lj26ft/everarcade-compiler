# Runtime Lifecycle

A world moves through a repeatable lifecycle: package, run, record, verify, checkpoint, restore, and continue.

```mermaid
flowchart LR
  Package --> Run --> Record --> Replay --> Verify --> Checkpoint --> Restore --> Continue
  Continue --> Run
```

The lifecycle preserves continuity while allowing operators and contributors to validate what happened.
