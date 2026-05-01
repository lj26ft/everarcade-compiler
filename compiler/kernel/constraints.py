FORBIDDEN = [
    "rand::",
    "SystemTime",
    "unwrap(",
    "panic!",
]


def enforce_constraints(content: str):
    for f in FORBIDDEN:
        if f in content:
            raise Exception(f"Forbidden pattern: {f}")
