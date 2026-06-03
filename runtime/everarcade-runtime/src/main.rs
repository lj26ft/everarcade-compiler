use anyhow::Result;
use everarcade_runtime::{OperatorCommand, RuntimeConfiguration, RuntimeOperator};
use std::env;
use std::path::PathBuf;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 5 {
        eprintln!("usage: runtime <command> <root> <world-id> <package-path>");
        std::process::exit(2);
    }
    let command = OperatorCommand::parse(&args[1])?;
    let config =
        RuntimeConfiguration::new(PathBuf::from(&args[2]), &args[3], PathBuf::from(&args[4]));
    let out = RuntimeOperator::new(config).execute(command)?;
    println!("{out}");
    Ok(())
}
