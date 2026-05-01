from kernel.frontend import build_ir
from kernel.ir import validate_ir
from kernel.backend import compile_plan, export_plan


def run(goal: str):
    print("🧠 EverArcade Compiler Kernel v1")

    ir = build_ir(goal)
    validate_ir(ir)

    plan = compile_plan(ir)

    export_plan(plan)

    print("✅ execution_plan.json generated")


if __name__ == "__main__":
    import sys
    goal = sys.argv[1] if len(sys.argv) > 1 else "build execution-core"

    run(goal)
