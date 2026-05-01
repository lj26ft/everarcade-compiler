from typing import List, Dict
from dataclasses import dataclass, field


@dataclass
class IRNode:
    id: str
    deps: List[str]
    outputs: List[str]
    requires_crates: List[str] = field(default_factory=list)


@dataclass
class CompilerIR:
    goal: str
    nodes: List[IRNode]


def validate_ir(ir: CompilerIR):
    ids = {n.id for n in ir.nodes}

    for node in ir.nodes:
        for dep in node.deps:
            if dep not in ids:
                raise Exception(f"Invalid dependency: {dep}")
