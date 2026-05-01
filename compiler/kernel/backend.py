import json
import os


def compile_plan(ir):
    return {
        "version": 1,
        "nodes": [
            {
                "id": "node_a",
                "deps": [],
                "payload": {
                    "type": "const",
                    "value": "hello"
                }
            },
            {
                "id": "node_b",
                "deps": ["node_a"],
                "payload": {
                    "type": "concat",
                    "with": " world"
                }
            }
        ]
    }


def export_plan(plan, filename="execution_plan.json"):
    base_dir = os.path.dirname(os.path.abspath(__file__))
    root_dir = os.path.dirname(base_dir)

    full_path = os.path.join(root_dir, filename)

    with open(full_path, "w") as f:
        json.dump(plan, f, indent=2)

    print(f"📄 Plan written to: {full_path}")
