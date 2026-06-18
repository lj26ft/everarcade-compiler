use canonicalizer_kernel::{canonicalize, state_root, world_hash, ArenaState};
use serde::Serialize;
use std::env;
use std::io::{self, Read};

#[derive(Serialize)]
struct StateOutput {
    canonical_hex: String,
    canonical_utf8: String,
    state_root: String,
    receipt_root: String,
    continuity_root: String,
    world_hash: String,
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read stdin");
    let command = env::args().nth(1).unwrap_or_else(|| "state".to_string());
    match command.as_str() {
        "state" => {
            let state: ArenaState = serde_json::from_str(&input).expect("parse ArenaState JSON");
            let bytes = canonicalize(&state);
            let root = state_root(&state);
            let output = StateOutput {
                canonical_hex: hex::encode(&bytes),
                canonical_utf8: String::from_utf8(bytes).expect("canonical JSON is UTF-8"),
                state_root: root.clone(),
                receipt_root: state.receipts.receipt_root.clone(),
                continuity_root: state.continuity.continuity_root.clone(),
                world_hash: world_hash(
                    &root,
                    &state.receipts.receipt_root,
                    &state.continuity.continuity_root,
                ),
            };
            println!(
                "{}",
                serde_json::to_string_pretty(&output).expect("serialize output")
            );
        }
        "world-hash" => {
            let roots: Vec<String> =
                serde_json::from_str(&input).expect("parse [state, receipt, continuity]");
            if roots.len() != 3 {
                panic!("world-hash expects exactly three roots");
            }
            println!("{}", world_hash(&roots[0], &roots[1], &roots[2]));
        }
        other => panic!("unsupported command: {other}"),
    }
}
