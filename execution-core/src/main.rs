//
// EVERARCADE EXECUTION TEST HARNESS (ABI v2 BINARY SAFE)
// NO JSON - STRICT TYPING ONLY
//

use execution_core::{
    ExecutionNode,
    ExecutionPlan,
    State,
    VmInput,
};

use std::collections::BTreeMap;

fn main() {
    println!("=== DETERMINISTIC WASM EXECUTION (ABI v2) ===\n");

    let mut state = State::new();

    let plan = ExecutionPlan {
        nodes: vec![
            ExecutionNode {
                id: "a".to_string(),
                action: "set".to_string(),

                payload: {
                    let mut map = BTreeMap::new();
                    map.insert("key".to_string(), "counter".to_string());
                    map.insert("value".to_string(), "5".to_string());
                    map
                },

                deps: vec![],
            },

            ExecutionNode {
                id: "b".to_string(),
                action: "increment".to_string(),

                payload: {
                    let mut map = BTreeMap::new();
                    map.insert("key".to_string(), "counter".to_string());
                    map.insert("amount".to_string(), "1".to_string());
                    map
                },

                deps: vec!["a".to_string()],
            },
        ],
    };

    let input = VmInput { state, plan };

    // NOTE:
    // This file is ONLY for construction/testing.
    // Actual execution happens in WASM runner.

    println!("Input constructed successfully.");
}
