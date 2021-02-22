install:
	cargo install wasm-pack
build:
	cd todomvc && wasm-pack build --target web --out-name wasm --out-dir ../static
run: build
	cargo run --release