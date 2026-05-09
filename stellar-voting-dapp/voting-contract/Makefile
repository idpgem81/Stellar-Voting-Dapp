default: build

all: test build

test:
	cargo test

build:
	cargo build --target wasm32-unknown-unknown --release
	soroban contract optimize --wasm ../../target/wasm32-unknown-unknown/release/voting-contract.wasm

clean:
	cargo clean