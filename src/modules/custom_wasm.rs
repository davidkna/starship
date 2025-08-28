use wasmtime::{component::bindgen, Config, Engine, Store};

pub fn module(context: &Context) -> Option<Module> {
    let engine = Engine::new(&Config::default()).ok()?;
    let store = Store::new(&engine);
    let component = bindgen::component(&store, "custom_wasm").ok()?;
    Some(Module::new(context, component))
}