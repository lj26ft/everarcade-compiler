use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use wasmtime::{Config, Engine, Linker, Module, Store};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct GuestPosition {
    pub x: i64,
    pub y: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct GuestOutput {
    pub action: String,
    pub player_id: String,
    pub position: GuestPosition,
    pub score: i64,
}

impl GuestOutput {
    pub fn canonical_bytes(&self) -> Result<Vec<u8>> {
        Ok(serde_json::to_vec(self)?)
    }

    pub fn stable_hash(&self) -> Result<String> {
        Ok(hex::encode(Sha256::digest(self.canonical_bytes()?)))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GuestInvocation {
    pub guest_id: String,
    pub entrypoint: String,
    pub actions: Vec<String>,
}

impl GuestInvocation {
    pub fn canonical(guest_id: impl Into<String>) -> Self {
        Self {
            guest_id: guest_id.into(),
            entrypoint: "everarcade_guest_execute".into(),
            actions: vec!["PlayerJoin".into(), "PlayerMove".into(), "ScoreUpdate".into()],
        }
    }

    pub fn canonical_bytes(&self) -> Result<Vec<u8>> {
        Ok(serde_json::to_vec(self)?)
    }

    pub fn stable_hash(&self) -> Result<String> {
        Ok(hex::encode(Sha256::digest(self.canonical_bytes()?)))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GuestExecutionResult {
    pub guest_id: String,
    pub guest_hash: String,
    pub invocation: GuestInvocation,
    pub input_hash: String,
    pub output: GuestOutput,
    pub guest_output_hash: String,
}

pub struct GuestWasmRunner;

impl GuestWasmRunner {
    pub fn execute(guest_id: &str, wasm: &[u8]) -> Result<GuestExecutionResult> {
        if !wasm.starts_with(b"\0asm") {
            return Err(anyhow!("world.wasm is not a WebAssembly binary"));
        }
        let mut config = Config::new();
        config.wasm_simd(false);
        config.wasm_relaxed_simd(false);
        config.wasm_tail_call(false);
        config.consume_fuel(true);
        let engine = Engine::new(&config)?;
        let module = Module::from_binary(&engine, wasm)?;
        let mut store = Store::new(&engine, ());
        store.set_fuel(1_000_000)?;
        let linker = Linker::new(&engine);
        let instance = linker.instantiate(&mut store, &module)?;
        let memory = instance
            .get_memory(&mut store, "memory")
            .ok_or_else(|| anyhow!("guest wasm missing exported memory"))?;
        let alloc = instance.get_typed_func::<i32, i32>(&mut store, "alloc")?;
        let entry = instance
            .get_typed_func::<(i32, i32), i64>(&mut store, "everarcade_guest_execute")
            .or_else(|_| instance.get_typed_func::<(i32, i32), i64>(&mut store, "everarcade_execute"))?;

        let invocation = GuestInvocation::canonical(guest_id);
        let input = invocation.canonical_bytes()?;
        let input_hash = invocation.stable_hash()?;
        let ptr = alloc.call(&mut store, i32::try_from(input.len())?)?;
        memory.write(&mut store, ptr as usize, &input)?;
        let handle = entry.call(&mut store, (ptr, i32::try_from(input.len())?))? as u64;
        let out_ptr = (handle >> 32) as usize;
        let out_len = (handle & 0xffff_ffff) as usize;
        if out_ptr == 0 || out_len == 0 || out_len > 64 * 1024 {
            return Err(anyhow!("guest returned an invalid output handle"));
        }
        let mut bytes = vec![0_u8; out_len];
        memory.read(&mut store, out_ptr, &mut bytes)?;
        let output: GuestOutput = serde_json::from_slice(&bytes)?;
        let guest_output_hash = output.stable_hash()?;
        Ok(GuestExecutionResult {
            guest_id: guest_id.to_string(),
            guest_hash: hex::encode(Sha256::digest(wasm)),
            invocation,
            input_hash,
            output,
            guest_output_hash,
        })
    }
}
