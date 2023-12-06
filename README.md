
```bash
$ rustup target add wasm32-unknown-unknown
$ cargo install wasm-bindgen-cli
$ cargo install basic-http-server
$ cargo build --profile wasm-release --target wasm32-unknown-unknown
$ wasm-bindgen --out-name jam4 \
  --out-dir wasm/target \
  --target web target/wasm32-unknown-unknown/wasm-release/bevy-jam-4.wasm
$ basic-http-server wasm
```