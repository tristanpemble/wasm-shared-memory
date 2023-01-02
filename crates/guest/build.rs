pub fn main() {
    // This ensures that the WASM module imports its linear memory rather than creating a new one.
    println!("cargo:rustc-link-arg=--import-memory");
}