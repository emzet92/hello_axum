mod domain;

use wasmtime::*;

// #[tokio::main]
use wasmtime::{Caller, Memory};

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

    linker.func_wrap(
        "env",
        "log_utf16",
        |mut caller: Caller<'_, ()>, ptr: i32, len: i32| {
            // 🔥 pobierz pamięć wasm
            let memory = match caller.get_export("memory") {
                Some(wasmtime::Extern::Memory(mem)) => mem,
                _ => {
                    println!("No memory export found!");
                    return;
                }
            };

            // 🔥 zakres bajtów (UTF-16 = 2 bajty na znak)
            let start = ptr as usize;
            let byte_len = (len as usize) * 2;

            let data = memory.data(&caller);

            let bytes = &data[start..start + byte_len];

            // 🔥 konwersja UTF-16 → Rust String
            let utf16: Vec<u16> = bytes
                .chunks_exact(2)
                .map(|b| u16::from_le_bytes([b[0], b[1]]))
                .collect();

            match String::from_utf16(&utf16) {
                Ok(s) => println!("WASM LOG: {}", s),
                Err(e) => println!("Invalid UTF-16: {:?}", e),
            }
        },
    )?;

    linker.func_wrap(
        "env",
        "abort",
        |mut caller: Caller<'_, ()>, msg_ptr: i32, file_ptr: i32, line: i32, col: i32| {
            let memory = match caller.get_export("memory") {
                Some(wasmtime::Extern::Memory(mem)) => mem,
                _ => {
                    println!("No memory found in abort");
                    return;
                }
            };

            let data = memory.data(&caller);

            // helper do czytania stringa UTF-16
            fn read_string(data: &[u8], ptr: i32) -> String {
                if ptr == 0 {
                    return "<null>".to_string();
                }

                // AssemblyScript string layout:
                // [length: i32][utf16 data...]
                let offset = ptr as usize;

                let len_bytes = &data[offset - 4..offset];
                let len = i32::from_le_bytes(len_bytes.try_into().unwrap()) as usize;

                let bytes = &data[offset..offset + len * 2];

                let utf16: Vec<u16> = bytes
                    .chunks_exact(2)
                    .map(|b| u16::from_le_bytes([b[0], b[1]]))
                    .collect();

                String::from_utf16(&utf16).unwrap_or("<invalid utf16>".into())
            }

            let msg = read_string(data, msg_ptr);
            let file = read_string(data, file_ptr);

            println!("🚨 WASM ABORT 🚨");
            println!("Message: {}", msg);
            println!("File: {}", file);
            println!("Line: {}, Column: {}", line, col);
        },
    )?;

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
