.PHONY: all
all: setup client server

.PHONY: setup
setup:
	cargo install wasm-pack

.PHONY: client
client:
	cd hoge_client &&\
	wasm-pack build  --target web --no-typescript --out-dir ../hoge_server/dist/js --out-name prpr

.PHONY: server
server:
	cd hoge_server &&\
	cargo run
