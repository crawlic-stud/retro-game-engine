run:
	cargo build & cargo run

build-wasm:
	cargo build -r --target wasm32-unknown-unknown
	cp target/wasm32-unknown-unknown/release/retro-game-engine.wasm retro-game-engine.wasm
