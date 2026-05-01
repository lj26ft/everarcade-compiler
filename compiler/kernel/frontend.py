from .ir import CompilerIR, IRNode


def build_ir(goal: str) -> CompilerIR:
    if "execution-core" in goal:
        return CompilerIR(
            goal="build_execution_core_v1",
            nodes=[
                IRNode(
                    id="init_crate",
                    deps=[],
                    outputs=["execution-core/Cargo.toml"]
                ),
                IRNode(
                    id="create_lib",
                    deps=["init_crate"],
                    outputs=["execution-core/src/lib.rs"]
                ),
                IRNode(
                    id="create_modules",
                    deps=["create_lib"],
                    outputs=[
                        "execution-core/src/dag.rs",
                        "execution-core/src/executor.rs",
                        "execution-core/src/state.rs"
                    ]
                ),
                IRNode(
                    id="add_crypto_stub",
                    deps=["create_modules"],
                    outputs=["execution-core/src/xrpl_signer.rs"]
                )
            ]
        )

    raise Exception("Unknown goal")
