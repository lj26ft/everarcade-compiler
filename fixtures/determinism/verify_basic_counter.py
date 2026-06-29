#!/usr/bin/env python3
"""Replay the public basic-counter determinism fixture."""
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent / "basic-counter-v1"


def canonical_bytes(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":")).encode("utf-8") + b"\n"


def state_root(state):
    return "sha256:" + hashlib.sha256(canonical_bytes(state)).hexdigest()


def apply_step(counter, step):
    action = step["action"]
    value = step["value"]
    if action == "add":
        return counter + value
    if action == "mul":
        return counter * value
    if action == "xor":
        return counter ^ value
    if action == "mod":
        return counter % value
    raise ValueError(f"unsupported action: {action}")


def main():
    inputs = json.loads((ROOT / "inputs.json").read_text(encoding="utf-8"))
    expected = json.loads((ROOT / "expected-roots.json").read_text(encoding="utf-8"))
    state = {"counter": inputs["initial_state"]["counter"], "log": []}
    actual_roots = []
    for step in inputs["steps"]:
        counter = apply_step(state["counter"], step)
        event = {"tick": step["tick"], "action": step["action"], "value": step["value"], "counter": counter}
        state = {"counter": counter, "log": state["log"] + [event]}
        actual_roots.append({"tick": step["tick"], "state_root": state_root(state)})
    if actual_roots != expected["step_roots"]:
        raise SystemExit(f"step root mismatch: {actual_roots} != {expected['step_roots']}")
    if state != expected["final_state"]:
        raise SystemExit("final state mismatch")
    if state_root(state) != expected["final_root"]:
        raise SystemExit("final root mismatch")
    print(expected["final_root"])


if __name__ == "__main__":
    main()
