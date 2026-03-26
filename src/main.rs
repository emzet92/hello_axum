mod domain;

use wasmtime::*;

// #[tokio::main]
use wasmtime::{Engine, Instance, Module, Store};

fn main() -> anyhow::Result<()> {
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());

    let module = Module::from_file(&engine, "build.wasm")?;

    let mut linker = Linker::new(&engine);

    // 🔥 bridge functions
    linker.func_wrap("env", "get_temp", || -> f64 { 35.0 })?;

    linker.func_wrap("env", "get_pressure", || -> f64 { 5.0 })?;

    linker.func_wrap("env", "log", || {
        println!("pierdoleniec :DD");
    })?;

    // zamiast Instance::new
    let instance = linker.instantiate(&mut store, &module)?;

    let evaluate = instance.get_typed_func::<(), i32>(&mut store, "evaluate")?;

    let result = evaluate.call(&mut store, ())?;

    println!("Result: {}", result != 0);

    // run void
    let run = instance.get_typed_func::<(), ()>(&mut store, "run")?;

    run.call(&mut store, ())?;

    Ok(())
}
