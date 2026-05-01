import json
import subprocess
import os
from typing import Dict, Any, List, Set
from jsonschema import validate, ValidationError

# =========================
# CONFIG
# =========================

MAX_RETRIES = 2

FORBIDDEN_PATTERNS = [
    "rand::",
    "SystemTime",
    "unwrap(",
    "panic!",
    "thread::",
    "tokio::",
]

# =========================
# SCHEMA (INLINE COPY)
# =========================

SCHEMA = {
    "type": "object",
    "required": ["goal", "nodes", "constraints"],
    "properties": {
        "goal": {"type": "string"},
        "constraints": {
            "type": "object",
            "required": [
                "no_randomness",
                "no_sys_time",
                "pure_functions_only"
            ],
            "properties": {
                "no_randomness": {"type": "boolean"},
                "no_sys_time": {"type": "boolean"},
                "pure_functions_only": {"type": "boolean"}
            }
        },
        "nodes": {
            "type": "array",
            "items": {
                "type": "object",
                "required": ["id", "action", "dependencies"],
                "properties": {
                    "id": {"type": "string"},
                    "dependencies": {
                        "type": "array",
                        "items": {"type": "string"}
                    },
                    "action": {
                        "type": "object",
                        "required": ["type"],
                        "properties": {
                            "type": {
                                "type": "string",
                                "enum": [
                                    "create_file",
                                    "write_code",
                                    "append_code",
                                    "cargo_init",
                                    "cargo_add",
                                    "cargo_check"
                                ]
                            },
                            "path": {"type": "string"},
                            "content": {"type": "string"},
                            "crate_name": {"type": "string"},
                            "dependencies": {
                                "type": "array",
                                "items": {"type": "string"}
                            }
                        }
                    }
                }
            }
        }
    }
}

# =========================
# CORE COMPILER AGENT
# =========================

class EverArcadeAgentV8:

    def __init__(self, llm):
        self.llm = llm

    # -------------------------
    # MAIN ENTRY
    # -------------------------

    def run(self, goal: str) -> Dict[str, Any]:
        plan = self._generate_plan(goal)

        plan = self._validate_and_repair(plan, goal)

        self._validate_determinism(plan)

        self._execute_dag(plan)

        return plan

    # -------------------------
    # LLM CALL
    # -------------------------

    def _generate_plan(self, goal: str) -> Dict[str, Any]:
        system_prompt = open("agent_v8_system_prompt.txt").read()

        response = self.llm(
            system_prompt=system_prompt,
            user=goal
        )

        return self._parse_json(response)

    # -------------------------
    # JSON PARSING (STRICT)
    # -------------------------

    def _parse_json(self, text: str) -> Dict[str, Any]:
        try:
            return json.loads(text)
        except Exception:
            raise ValueError("Invalid JSON output from LLM")

    # -------------------------
    # VALIDATION + REPAIR LOOP
    # -------------------------

    def _validate_and_repair(self, plan: Dict[str, Any], goal: str) -> Dict[str, Any]:
        for attempt in range(MAX_RETRIES + 1):
            try:
                validate(instance=plan, schema=SCHEMA)
                return plan
            except ValidationError as e:
                plan = self._repair_plan(goal, str(e), plan)

        raise Exception("Failed to produce valid plan after retries")

    def _repair_plan(self, goal: str, error: str, last_plan: Dict[str, Any]) -> Dict[str, Any]:
        repair_prompt = f"""
Fix this execution plan to match schema exactly.

ERROR:
{error}

PREVIOUS PLAN:
{json.dumps(last_plan, indent=2)}

GOAL:
{goal}

Return ONLY valid JSON.
"""

        response = self.llm(system_prompt="REPAIR MODE", user=repair_prompt)
        return self._parse_json(response)

    # -------------------------
    # DETERMINISM CHECK
    # -------------------------

    def _validate_determinism(self, plan: Dict[str, Any]):
        plan_str = json.dumps(plan)

        for pattern in FORBIDDEN_PATTERNS:
            if pattern in plan_str:
                raise Exception(f"Determinism violation: {pattern}")

    # -------------------------
    # DAG EXECUTION
    # -------------------------

    def _execute_dag(self, plan: Dict[str, Any]):
        nodes = {n["id"]: n for n in plan["nodes"]}

        visited = set()
        stack = []

        def visit(node_id: str):
            if node_id in visited:
                return

            node = nodes[node_id]

            for dep in node["dependencies"]:
                visit(dep)

            self._execute_node(node)
            visited.add(node_id)

        for node_id in nodes:
            visit(node_id)

    # -------------------------
    # NODE EXECUTION
    # -------------------------

    def _execute_node(self, node: Dict[str, Any]):
        action = node["action"]
        t = action["type"]

        print(f"▶ Executing node: {node['id']} :: {t}")

        if t == "cargo_init":
            self._cargo_init(action["crate_name"])

        elif t == "write_code":
            self._write_file(action["path"], action.get("content", ""))

        elif t == "append_code":
            self._append_file(action["path"], action.get("content", ""))

        elif t == "cargo_add":
            self._cargo_add(action.get("dependencies", []))

        elif t == "cargo_check":
            self._cargo_check()

        elif t == "create_file":
            self._write_file(action["path"], "")

        else:
            raise Exception(f"Unknown action type: {t}")

    # -------------------------
    # RUNTIME OPERATIONS
    # -------------------------

    def _cargo_init(self, name: str):
        subprocess.run(["cargo", "new", name], check=False)

    def _cargo_add(self, deps: List[str]):
        for dep in deps:
            subprocess.run(["cargo", "add", dep], check=False)

    def _cargo_check(self):
        subprocess.run(["cargo", "check"], check=False)

    def _write_file(self, path: str, content: str):
        os.makedirs(os.path.dirname(path), exist_ok=True)
        with open(path, "w") as f:
            f.write(content)

    def _append_file(self, path: str, content: str):
        with open(path, "a") as f:
            f.write(content)


# =========================
# ENTRY POINT
# =========================

if __name__ == "__main__":
    def mock_llm(system_prompt, user):
        # Replace with Ollama / Claude / OpenAI
        return '{"goal":"mock","constraints":{"no_randomness":true,"no_sys_time":true,"pure_functions_only":true},"nodes":[]}'

    agent = EverArcadeAgentV8(llm=mock_llm)

    import sys
    goal = sys.argv[1] if len(sys.argv) > 1 else "build execution-core v1"

    result = agent.run(goal)

    print(json.dumps(result, indent=2))
