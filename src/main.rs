use wasmtime::component::*;
use wasmtime::{Config, Engine, Store};

bindgen!();

fn main() -> wasmtime::Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;
    let component = Component::from_file(&engine, "../../manual/addcomp.wasm")?;

    let linker = Linker::new(&engine);

    let mut store = Store::new(&engine, ());

    let (bindings, _) = Root::instantiate(&mut store, &component, &linker)?;

    // let res = bindings.call_entry(&mut store, "example string here")?;
    let res = bindings.call_length(store, "example string here")?;
    println!("{res}");
    Ok(())
}
