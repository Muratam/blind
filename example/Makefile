.PHONY: all
all: setup wasm server

.PHONY: setup
setup:
	cargo install wasm-pack

.PHONY: wasm
wasm:
	cd wasm &&\
	wasm-pack build --target web --no-typescript --out-dir ../server/dist/js --out-name strattera

.PHONY: server
server:
	cd server &&\
	cargo run
