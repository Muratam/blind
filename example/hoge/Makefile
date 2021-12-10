root_name := hoge

.PHONY: all
all: setup client server

.PHONY: setup
setup:
	cargo install wasm-pack -q

.PHONY: client
client:
	cd $(root_name)_client &&\
	wasm-pack build  --target web --no-typescript --out-dir ../$(root_name)_server/dist/js --out-name prpr
  #  --profiling / --release

.PHONY: server
server:
	cd $(root_name)_server &&\
	cargo run
