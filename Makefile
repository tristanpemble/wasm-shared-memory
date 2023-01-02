default: ./target/release/host
	exec ./target/release/host
./target/release/host: ./target/wasm32-unknown-unknown/release/guest.wasm
	exec cargo build -p host --release
./target/wasm32-unknown-unknown/release/guest.wasm:
	exec cargo build -p guest --target wasm32-unknown-unknown --release
