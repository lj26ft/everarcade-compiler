from typing import List
from .ir import IRNode


def topo_sort(nodes: List[IRNode]) -> List[IRNode]:
    visited = set()
    order = []

    node_map = {n.id: n for n in nodes}

    def visit(n: IRNode):
        if n.id in visited:
            return
        for dep in n.deps:
            visit(node_map[dep])
        visited.add(n.id)
        order.append(n)

    for node in nodes:
        visit(node)

    return order
