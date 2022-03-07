build-bin:
	cargo build --release --bin ethereal

build-wasm:
	cargo build --release --bin wasm --target wasm32-unknown-unknown
