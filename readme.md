# Strattera

## run example

1. build client wasm binary

```sh
$ cargo install wasm-pack
$ cd impl/client
$ wasm-pack build --target web --no-typescript
```

2. run example server binary

```sh
$ cd impl/server
$ cargo run --example server
```
