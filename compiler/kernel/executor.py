import subprocess
import os


def execute_plan(plan):
    for step in plan:
        print(f"▶ {step['action']}")

        if step["action"] == "cargo_init":
            subprocess.run(["cargo", "new", step["crate"]], check=False)

        elif step["action"] == "create_file":
            os.makedirs(os.path.dirname(step["path"]), exist_ok=True)
            open(step["path"], "w").close()

        elif step["action"] == "write_file":
            os.makedirs(os.path.dirname(step["path"]), exist_ok=True)
            with open(step["path"], "w") as f:
                f.write(step["content"])
