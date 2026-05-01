// FILE: execution-core/src/main.rs
//
// LOCAL NATIVE TEST RUNNER
//
// PURPOSE:
// - native testing
// - deterministic execution verification
// - WASM-independent debugging
//
// RUN:
// cargo run -p execution-core
//

use execution_core::{
    ExecutionNode,
    ExecutionPlan,
    State,
    VmInput,
};

fn main() {
    //
    // ========================================================
    // INITIAL STATE
    // ========================================================
    //

    let state = State::new();

    //
    // ========================================================
    // EXECUTION PLAN
    // ========================================================
    //

    let plan = ExecutionPlan {
        nodes: vec![
            ExecutionNode {
                id: "a".to_string(),
                action: "set".to_string(),
                payload: serde_json::json!({
                    "key": "counter",
                    "value": "5"
                }),
                deps: vec![],
            },
            ExecutionNode {
                id: "b".to_string(),
                action: "increment".to_string(),
                payload: serde_json::json!({
                    "key": "counter",
                    "amount": 1
                }),
                deps: vec!["a".to_string()],
            },
        ],
    };

    //
    // ========================================================
    // VM INPUT
    // ========================================================
    //

    let input = VmInput {
        state,
        plan,
    };

    //
    // ========================================================
    // EXECUTE VM
    // ========================================================
    //

    let output =
        execution_core::execute::execute_vm(input);

    //
    // ========================================================
    // DISPLAY RECEIPT
    // ========================================================
    //

    println!();
    println!("PREVIOUS STATE ROOT:");
    println!(
        "{}",
        output.receipt.previous_state_root
    );

    println!();
    println!("NEW STATE ROOT:");
    println!(
        "{}",
        output.receipt.new_state_root
    );

    println!();
    println!("EXECUTION ROOT:");
    println!(
        "{}",
        output.receipt.execution_root
    );

    println!();
    println!("RECEIPT HASH:");
    println!(
        "{}",
        output.receipt.receipt_hash
    );

    println!();
    println!("NODE HASHES:");

    for (id, hash) in &output.receipt.node_hashes {
        println!("{} => {}", id, hash);
    }

    println!();
    println!("STATE CHANGES:");

    for change in &output.receipt.state_changes {
        println!(
            "{}: '{}' -> '{}'",
            change.key,
            change.before,
            change.after
        );
    }

    println!();
    println!("FINAL STATE:");

    for (key, value) in &output.updated_state {
        println!("{} => {}", key, value);
    }

    println!();
}
