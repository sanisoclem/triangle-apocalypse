
```bash
$ rustup target add wasm32-unknown-unknown
$ cargo install wasm-bindgen-cli
$ cargo install basic-http-server
$ RUSTFLAGS=--cfg=web_sys_unstable_apis cargo build --profile wasm-release --target wasm32-unknown-unknown
$ wasm-bindgen --out-name jam4 \
  --out-dir wasm/target \
  --target web target/wasm32-unknown-unknown/wasm-release/bevy-jam-4.wasm
$ basic-http-server wasm
```

[WASM](https://shape-shepherd-sanisoclem.vercel.app/)

Known issues:
 - wasm build uses webgpu (needed for compute shaders created by `bevy_hanabi`) and only works in chrome
 - particle effects in Chrome in windows is buggy
