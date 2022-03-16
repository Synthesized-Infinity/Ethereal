build:
	cargo build --release --bin ethereal 

build-wasm:
	cargo build --release --bin wasm --target wasm32-unknown-unknown

build-linux:
	cargo build --release --target x86_64-unknown-linux-gnu
