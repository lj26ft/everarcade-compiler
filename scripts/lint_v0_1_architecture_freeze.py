#!/usr/bin/env python3
"""Deterministic adversarial linter for the v0.1 architecture freeze."""
from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
FREEZE_DOC = ROOT / "docs" / "14-v0.1-architecture-freeze.md"
CANONICAL_DOCS = [
    ROOT / "docs" / "03-system-architecture.md",
    ROOT / "docs" / "10-release-certification.md",
    ROOT / "docs" / "11-production-readiness.md",
    ROOT / "docs" / "12-gap-analysis.md",
    FREEZE_DOC,
    ROOT / "MATURITY.md",
]
SCAFFOLD_DOMAINS = ("renderer", "history", "federation")
FORBIDDEN_PRODUCTION_PATTERNS = [
    re.compile(r"\bproduction[- ]ready\b", re.IGNORECASE),
    re.compile(r"\bcommercially ready\b", re.IGNORECASE),
    re.compile(r"\blive settlement\b", re.IGNORECASE),
]
REQUIRED_FREEZE_PHRASES = [
    "v0.1 architecture freeze",
    "Tier 1",
    "Tier 2",
    "deterministic adversarial linter",
    "single-world runtime",
    "renderer/history/federation",
    "scaffold-level runtime domains",
]


def read(path: Path) -> str:
    try:
        return path.read_text(encoding="utf-8")
    except FileNotFoundError:
        raise AssertionError(f"missing required file: {path.relative_to(ROOT)}")


def has_final_newline(path: Path) -> bool:
    return path.read_bytes().endswith(b"\n")


def line_of(text: str, offset: int) -> int:
    return text.count("\n", 0, offset) + 1


def tier1() -> list[str]:
    failures: list[str] = []
    freeze = read(FREEZE_DOC)
    for phrase in REQUIRED_FREEZE_PHRASES:
        if phrase not in freeze:
            failures.append(f"freeze doc missing required phrase: {phrase}")
    for path in CANONICAL_DOCS:
        text = read(path)
        if not has_final_newline(path):
            failures.append(f"{path.relative_to(ROOT)} must end with a newline")
    return failures


def tier2() -> list[str]:
    failures: list[str] = []
    for path in CANONICAL_DOCS:
        text = read(path)
        rel = path.relative_to(ROOT)
        for pattern in FORBIDDEN_PRODUCTION_PATTERNS:
            for match in pattern.finditer(text):
                line = line_of(text, match.start())
                window = text[max(0, match.start() - 120): match.end() + 120].lower()
                if "not " not in window and "no subsystem" not in window and "unless" not in window and "production ready: yes" not in window and "production ready |" not in window and "only when" not in window:
                    failures.append(f"{rel}:{line}: unqualified production claim: {match.group(0)!r}")
    freeze = read(FREEZE_DOC).lower()
    for domain in SCAFFOLD_DOMAINS:
        if domain not in freeze:
            failures.append(f"freeze doc does not explicitly name scaffold domain: {domain}")
    if "must not" not in freeze:
        failures.append("freeze doc must include at least one explicit must-not boundary")
    return failures


def main() -> int:
    failures = tier1() + tier2()
    if failures:
        print("v0.1 architecture freeze linter: FAIL")
        for failure in failures:
            print(f"- {failure}")
        return 1
    print("v0.1 architecture freeze linter: PASS")
    print("- Tier 1 canonical freeze checks passed")
    print("- Tier 2 adversarial claim-boundary checks passed")
    return 0


if __name__ == "__main__":
    sys.exit(main())
