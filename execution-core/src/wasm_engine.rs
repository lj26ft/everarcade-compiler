use crate::types::{
    State,
    StateChange,
};

use wasmtime::*;

pub struct WasmEngine {
    engine: Engine,
}

impl WasmEngine {
    pub fn new() -> Self {
        Self {
            engine: Engine::default(),
        }
    }

    pub fn execute_contract(
        &self,
        wasm_path: &str,
        state: &mut State,
    ) -> Vec<StateChange> {

        let module =
            Module::from_file(
                &self.engine,
                wasm_path,
            )
            .unwrap();

        let mut store =
            Store::new(
                &self.engine,
                (),
            );

        let instance =
            Instance::new(
                &mut store,
                &module,
                &[],
            )
            .unwrap();

        let execute =
            instance
                .get_typed_func::<(), i32>(
                    &mut store,
                    "execute",
                )
                .unwrap();

        let result =
            execute
                .call(
                    &mut store,
                    (),
                )
                .unwrap();

        let before =
            state
                .get("counter")
                .cloned()
                .unwrap_or_default();

        state.insert(
            "counter".to_string(),
            result.to_string(),
        );

        vec![
            StateChange {
                key: "counter".to_string(),
                before,
                after: result.to_string(),
            }
        ]
    }
}
