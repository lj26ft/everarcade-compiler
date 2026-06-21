# Arena Vanguard Replay Verification

`everarcade replay-world` replays `runtime/games/arena-vanguard/journal.json` from genesis and compares the resulting proof bundle with the recorded bundle.

Verification proves equality for:

- state root
- receipt root
- world hash
- continuity root

A mismatch rejects the replay as divergence.
