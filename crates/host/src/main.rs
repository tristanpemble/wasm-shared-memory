use anyhow::Result;
use wasmtime::*;

fn main() -> Result<()> {
    // Spin up the wasmtime components.
    let engine = Engine::default();
    let mut store = Store::new(&engine, 0);
    let mut linker = Linker::new(&engine);

    // Define a linear memory, and make it available for import via the linker.
    let mem = Memory::new(&mut store, MemoryType::new(17, None))?;
    linker.define("env", "memory", mem)?;

    // Expose a dumb print function inside the WASM instances.
    linker.func_wrap("env", "print", move |caller: Caller<'_, i32>, _: i32, ptr: i32, length: i32| {
        let slice = mem.data(&caller);
        let ptr = ptr as usize;
        let length = length as usize;
        let str = String::from_utf8_lossy(&slice[ptr..ptr+length]);
        println!("{}", str)
    })?;

    // Instantiate several instances of the WASM module.
    let module = Module::from_file(&engine, "./target/wasm32-unknown-unknown/release/guest.wasm")?;
    let i0 = linker.instantiate(&mut store, &module)?;
    let i1 = linker.instantiate(&mut store, &module)?;
    let i2 = linker.instantiate(&mut store, &module)?;

    // Allocate the value 11 in instance 0.
    let ptr = i0.get_typed_func::<i32, i32>(&mut store, "alloc_11")?.call(&mut store, 0)?;

    for _ in 0..5 {
        // Add 23 to the variable in instance 0.
        i0.get_typed_func::<(i32, i32), ()>(&mut store, "add_23")?.call(&mut store, (0, ptr))?;
        // Add 23 to the variable in instance 1.
        i1.get_typed_func::<(i32, i32), ()>(&mut store, "add_23")?.call(&mut store, (1, ptr))?;
        // Add 23 to the variable in instance 2.
        i2.get_typed_func::<(i32, i32), ()>(&mut store, "add_23")?.call(&mut store, (2, ptr))?;
    }

    println!("expected: 356");

    Ok(())
}
