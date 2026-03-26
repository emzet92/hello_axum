mod domain;


use wasmtime::*;


// #[tokio::main]
use wasmtime::{Engine, Module, Store, Instance};

fn main() -> anyhow::Result<()> {
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());

    let module = Module::from_file(&engine, "build.wasm")?;

    // 👉 TO MUSI BYĆ
    let instance = Instance::new(&mut store, &module, &[])?;

    // 👉 teraz dopiero działa
    let evaluate = instance
        .get_typed_func::<(f64, f64), i32>(&mut store, "evaluate")?;

    let result = evaluate.call(&mut store, (35.0, 11.0))?;

    println!("Result: {}", result != 0);
    println!("Result2: {}", result);

    Ok(())
}
