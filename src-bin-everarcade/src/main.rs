mod arena_vanguard;
mod commands;
mod config;
mod lease;
mod operator_identity;
mod product;
mod release;
mod runtime_snapshot;
mod world;
use std::env;

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(1);
    }
}
fn run() -> Result<(), String> {
    let mut args: Vec<String> = env::args().collect();
    let invoked_as_world_evr = args
        .get(0)
        .and_then(|p| std::path::Path::new(p).file_stem())
        .and_then(|s| s.to_str())
        == Some("world-evr");
    if invoked_as_world_evr {
        args.insert(1, "world".to_string());
    }
    if matches!(
        args.get(1).map(String::as_str),
        Some("help" | "--help" | "-h") | None
    ) {
        commands::print_help();
        return Ok(());
    }
    if matches!(args.get(1).map(String::as_str), Some("runtime-snapshot")) {
        return runtime_snapshot::runtime_snapshot(
            args.get(2).map(String::as_str).unwrap_or("runtime/config"),
        );
    }
    if let Some(cmd) = args.get(1).map(String::as_str) {
        if product::is_product_command(cmd) {
            return product::dispatch(&args);
        }
    }
    commands::dispatch(&args).or_else(|e| {
        commands::print_help();
        Err(e)
    })
}
