use wasmtime::{Instance, Linker, Module, Store};

pub fn instantiate(
    store: &mut Store<()>,
    linker: &Linker<()>,
    module: &Module,
) -> anyhow::Result<Instance> {
    Ok(linker.instantiate(store, module)?)
}
