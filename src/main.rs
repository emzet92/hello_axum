mod domain;

use std::collections::HashMap;
use wasmtime::{Caller, Engine, Linker, Module, Store};

#[derive(Default)]
struct HostState {
    sensors: HashMap<String, f64>,
}

fn read_utf16_from_memory(
    caller: &mut Caller<'_, HostState>,
    ptr: i32,
    len: i32,
) -> anyhow::Result<String> {
    let memory = match caller.get_export("memory") {
        Some(wasmtime::Extern::Memory(mem)) => mem,
        _ => anyhow::bail!("memory export not found"),
    };

    if ptr < 0 || len < 0 {
        anyhow::bail!("negative ptr or len");
    }

    let start = ptr as usize;
    let byte_len = (len as usize)
        .checked_mul(2)
        .ok_or_else(|| anyhow::anyhow!("byte length overflow"))?;

    let data = memory.data(&caller);

    let end = start
        .checked_add(byte_len)
        .ok_or_else(|| anyhow::anyhow!("memory range overflow"))?;

    if end > data.len() {
        anyhow::bail!("out of bounds memory read");
    }

    let bytes = &data[start..end];

    let utf16: Vec<u16> = bytes
        .chunks_exact(2)
        .map(|b| u16::from_le_bytes([b[0], b[1]]))
        .collect();

    let s = String::from_utf16(&utf16)?;
    Ok(s)
}

fn main() -> anyhow::Result<()> {
    let engine = Engine::default();

    let mut sensors = HashMap::new();
    sensors.insert("Motor1/Sensor/Temp".to_string(), 42.0);
    sensors.insert("Motor1/Sensor/Pressure".to_string(), 5.0);

    let host_state = HostState { sensors };
    let mut store = Store::new(&engine, host_state);

    let module = Module::from_file(&engine, "build.wasm")?;
    let mut linker = Linker::new(&engine);

    linker.func_wrap(
        "env",
        "get_value_utf16",
        |mut caller: Caller<'_, HostState>, ptr: i32, len: i32| -> f64 {
            let key = match read_utf16_from_memory(&mut caller, ptr, len) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("get_value_utf16: failed to read key: {e}");
                    return f64::NAN;
                }
            };

            match caller.data().sensors.get(&key) {
                Some(value) => *value,
                None => {
                    eprintln!("get_value_utf16: missing key: {}", key);
                    f64::NAN
                }
            }
        },
    )?;

    linker.func_wrap(
        "env",
        "log_utf16",
        |mut caller: Caller<'_, HostState>, ptr: i32, len: i32| match read_utf16_from_memory(
            &mut caller,
            ptr,
            len,
        ) {
            Ok(msg) => println!("WASM LOG: {}", msg),
            Err(e) => eprintln!("log_utf16: failed to read message: {e}"),
        },
    )?;

    linker.func_wrap(
        "env",
        "abort",
        |mut caller: Caller<'_, HostState>, msg_ptr: i32, file_ptr: i32, line: i32, col: i32| {
            let msg = read_asc_string_object(&mut caller, msg_ptr)
                .unwrap_or_else(|e| format!("<failed to read abort message: {e}>"));

            let file = read_asc_string_object(&mut caller, file_ptr)
                .unwrap_or_else(|e| format!("<failed to read abort file: {e}>"));

            eprintln!("WASM ABORT");
            eprintln!("Message: {}", msg);
            eprintln!("File: {}", file);
            eprintln!("Line: {}, Column: {}", line, col);
        },
    )?;

    let instance = linker.instantiate(&mut store, &module)?;

    let evaluate = instance.get_typed_func::<(), i32>(&mut store, "evaluate")?;
    let result = evaluate.call(&mut store, ())?;
    println!("Result: {}", result != 0);

    let run = instance.get_typed_func::<(), ()>(&mut store, "run")?;
    run.call(&mut store, ())?;

    Ok(())
}

fn read_asc_string_object(caller: &mut Caller<'_, HostState>, ptr: i32) -> anyhow::Result<String> {
    let memory = match caller.get_export("memory") {
        Some(wasmtime::Extern::Memory(mem)) => mem,
        _ => anyhow::bail!("memory export not found"),
    };

    if ptr <= 0 {
        anyhow::bail!("null or invalid string ptr");
    }

    let data = memory.data(&caller);

    let ptr_usize = ptr as usize;

    if ptr_usize < 4 {
        anyhow::bail!("string ptr too small");
    }

    if ptr_usize > data.len() {
        anyhow::bail!("string ptr out of bounds");
    }

    let len_start = ptr_usize - 4;
    let len_end = ptr_usize;

    if len_end > data.len() {
        anyhow::bail!("string length header out of bounds");
    }

    let len_bytes: [u8; 4] = data[len_start..len_end]
        .try_into()
        .map_err(|_| anyhow::anyhow!("invalid length header"))?;

    let len = i32::from_le_bytes(len_bytes);

    if len < 0 {
        anyhow::bail!("negative string length");
    }

    read_utf16_from_memory(caller, ptr, len)
}
