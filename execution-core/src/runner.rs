use std::fs;

use wasmtime::*;

fn main() -> anyhow::Result<()> {
    // Load WASM
    let engine = Engine::default();
    let module = Module::from_file(
        &engine,
        "target/wasm32-unknown-unknown/release/execution_core.wasm",
    )?;

    let mut store = Store::new(&engine, ());
    let instance = Instance::new(&mut store, &module, &[])?;

    // Get exports
    let alloc = instance.get_typed_func::<usize, u32>(&mut store, "alloc")?;
    let run = instance.get_typed_func::<(u32, usize), ()>(&mut store, "run")?;
    let result_ptr = instance.get_typed_func::<(), u32>(&mut store, "result_ptr")?;
    let result_len = instance.get_typed_func::<(), u32>(&mut store, "result_len")?;

    let memory = instance
        .get_memory(&mut store, "memory")
        .expect("no memory export");

    // Load execution plan
    let json = fs::read_to_string("../everarcade-compiler/execution_plan.json")?;
    let bytes = json.as_bytes();

    // Allocate in WASM
    let ptr = alloc.call(&mut store, bytes.len())?;

    // Write into WASM memory
    memory.write(&mut store, ptr as usize, bytes)?;

    // Run
    run.call(&mut store, (ptr, bytes.len()))?;

    // Read result
    let rptr = result_ptr.call(&mut store)?;
    let rlen = result_len.call(&mut store)?;

    let mut buffer = vec![0u8; rlen as usize];
    memory.read(&mut store, rptr as usize, &mut buffer)?;

    let result = String::from_utf8(buffer)?;
    println!("🌳 ROOT HASH (WASM): {}", result);

    Ok(())
}
