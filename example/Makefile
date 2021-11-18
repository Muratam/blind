.PHONY: all
all: setup client server

.PHONY: setup
setup:
	cargo install wasm-pack

.PHONY: client
client:
	cd client &&\
	wasm-pack build --target web --no-typescript --out-dir ../server/dist/js --out-name blind

.PHONY: server
server:
	cd server &&\
	cargo run
