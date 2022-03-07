build-bin:
	cargo build --release --bin ethereal-bin

build-wasm:
	cargo build --release --bin wasm --target wasm32-unknown-unknown
