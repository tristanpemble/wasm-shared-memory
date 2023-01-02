# wasm-shared-memory

# Explanation

this is just a proof of concept of how to share memory between WASM module instantiations with Rust and Wasmtime. a
similar approach would work with Wasmer.

the key thing that had to be done to make this work was a [custom build file](https://github.com/tristanpemble/wasm-shared-memory/blob/4d01483e9630771bc4cf1ac72c37235a9c0fdefa/crates/guest/build.rs#L3)
for the WASM guest module that adds the `--import-memory` [flag for LLD](https://lld.llvm.org/WebAssembly.html#cmdoption-import-memory).
enabling this flag  lets you link a [`wasmtime::memory::Memory`](https://docs.wasmtime.dev/api/wasmtime/struct.Memory.html)
when instantiating each module. AssemblyScript has a [similar compiler configuration](https://www.assemblyscript.org/compiler.html#compiler-options),
and I assume other languages that compile to WASM support the config as well. Rust's was just a little tricky to find.

the obvious caveat when doing this is that you're reducing isolation between module instances, however this may be a
desired trade-off in certain high performance situations where copies are unacceptable.

# Usage

run `make` in the project root. A small native binary will load an WASM three times. The module has two functions
exported: `alloc_11` and `add_23`. The former allocates an `i32` with the value 11. The latter adds `23` to the `i32` at
the passed in address in memory. The native code runs `alloc_11` once, then iterates through each instance running
`add_23` to demonstrate shared memory access and mutation.

```
λ make
exec ./target/release/host
alloc_11[#0]: 11
add_23[#0]: 34
add_23[#1]: 57
add_23[#2]: 80
add_23[#0]: 103
add_23[#1]: 126
add_23[#2]: 149
add_23[#0]: 172
add_23[#1]: 195
add_23[#2]: 218
add_23[#0]: 241
add_23[#1]: 264
add_23[#2]: 287
add_23[#0]: 310
add_23[#1]: 333
add_23[#2]: 356
expected: 356
```
