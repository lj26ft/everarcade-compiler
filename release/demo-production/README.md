# EverArcade Release Demo Production

This package converts the Public Demonstration Certification into a recorded and live-runnable demo.

## Deliverables

- Projection runtime UI polish: `runtime/games/arena-vanguard/projection/` includes the live projection dashboard and demo-mode fallback.
- Operator root dashboard: `operator-dashboard.html` presents controls, roots, federation health, and PASS status.
- Migration animation: the operator dashboard renders the freeze/export/transfer/resume sequence from `demo-world-seed.json`.
- Replay timeline: the operator dashboard renders replay ticks and roots from the seed state.
- Restore timeline: the operator dashboard renders checkpoint restore stages.
- Demo world seed state: `demo-world-seed.json` contains the deterministic five-minute demo world.
- Demo recording script: `recording-script.md` provides the narration and shot list.

## Live-runnable flow

```bash
python3 -m http.server 8080 --directory release/demo-production
# open http://127.0.0.1:8080/operator-dashboard.html
```

For the game projection, serve `runtime/games/arena-vanguard/projection` beside an authoritative runtime exposing `/state` and `/verify`; if those endpoints are absent, the page enters demo mode with the release seed data.

## Certification outcome

`RELEASE DEMO READY: PASS`
