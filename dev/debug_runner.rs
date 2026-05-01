use std::collections::BTreeMap;

use execution_core::execute::*;

fn main() {
    let state = State::new();

    let mut set_payload =
        BTreeMap::new();

    set_payload.insert(
        "key".to_string(),
        "counter".to_string(),
    );

    set_payload.insert(
        "value".to_string(),
        "5".to_string(),
    );

    let mut increment_payload =
        BTreeMap::new();

    increment_payload.insert(
        "key".to_string(),
        "counter".to_string(),
    );

    increment_payload.insert(
        "amount".to_string(),
        "1".to_string(),
    );

    let plan = ExecutionPlan {
        nodes: vec![
            ExecutionNode {
                id: "a".to_string(),
                contract: "set".to_string(),
                payload: set_payload,
            },
            ExecutionNode {
                id: "b".to_string(),
                contract:
                    "increment".to_string(),
                payload: increment_payload,
            },
        ],
    };

    let input = VmInput {
        state,
        plan,
    };

    let output =
        execute_vm(input);

    println!(
        "\n=== ABI v2 EXECUTION ==="
    );

    println!(
        "\nPREVIOUS STATE ROOT:\n{}",
        output
            .receipt
            .previous_state_root
    );

    println!(
        "\nNEW STATE ROOT:\n{}",
        output
            .receipt
            .new_state_root
    );

    println!(
        "\nEXECUTION ROOT:\n{}",
        output
            .receipt
            .execution_root
    );

    println!(
        "\nRECEIPT HASH:\n{}",
        output
            .receipt
            .receipt_hash
    );

    println!("\nFINAL STATE:");

    for (k, v) in output
        .updated_state
        .iter()
    {
        println!("{k} => {v}");
    }
}
