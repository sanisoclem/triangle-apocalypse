# Triangle Apocalypse

Available to play at [https://shape-shepherd-sanisoclem.vercel.app/](https://shape-shepherd-sanisoclem.vercel.app/), also see [known issues](#known-issues).

## Running from source

### Natively

```bash
# run in release
$ cargo run --release
# enable hot asset reloading
$ cargo run --release --feature hotreload
# enable hot reload and debug screens
$ cargo run --release --features="hotreload,debug"
```

### Wasm


Install prerequisites:
```bash
$ rustup target add wasm32-unknown-unknown
$ cargo install wasm-bindgen-cli
$ cargo install basic-http-server
```

```bash
$ RUSTFLAGS=--cfg=web_sys_unstable_apis cargo build --profile wasm-release --target wasm32-unknown-unknown
$ wasm-bindgen --out-name jam4 \
  --out-dir wasm/target \
  --target web target/wasm32-unknown-unknown/wasm-release/bevy-jam-4.wasm
$ basic-http-server wasm
```


## Known issues:
 - wasm build only works on chrome - webgpu is needed for compute shaders created by `bevy_hanabi`
 - particle effects are buggy when using Chrome on Windows
 - `.DS_Store` files in `/assets/preload` causes the app to crash. This happens on mac when browsing the assets folder in finder.

## Credits:
 - [bevy_hanabi](https://github.com/djeedai/bevy_hanabi) - particle effects (using a [fork](https://github.com/djeedai/bevy_hanabi/compare/main...sanisoclem:bevy_hanabi:wasm) to rebase `u/wasm`)
 - [bevy_smud](https://github.com/johanhelsing/bevy_smud) - render SDFs
 - [sdfu](https://github.com/fu5ha/sdfu) - signed distance field utilities for defining bounds and checking collision (using a [fork](https://github.com/fu5ha/sdfu/compare/master...sanisoclem:sdfu:master) with minor changes)
 - [pristine_grid](https://github.com/rust-adventure/bevy-examples/blob/61981736c1afed1bdca85c9a5599001774844c8d/libs/bevy_shader_utils/shaders/pristine_grid.wgsl#L4-L38) - from [Chris Biscardi's showcase](https://discord.com/channels/691052431525675048/692648638823923732/1169146926466334731), used in the background shader
 - https://freesound.org/people/colorsCrimsonTears/sounds/607409/ - boost sound effect
 - https://freesound.org/people/TheBuilder15/sounds/352351/ - wind down sound effect
 - https://convertio.co/wav-ogg/ - used to convert the above 2 files to `.ogg`
 - [www.beepbox.co](https://www.beepbox.co) - for prototyping music before editing in a DAW(see [prototype of main theme](https://www.beepbox.co/#9n41sbk0l00e0ft2ma7g0vj0ar1i0o4332T1v2u56f0qwx10p711d03A5F5B9Q0001PfaedE4b762663777T1v2u65f0q0x10t51d08A1F2B9Q00d0Pfc47E3b662878T1v1u19f0q802d23A5F4B0Q0202PeebbE0T1v4ue1f0q0y10n73d4aA0F0B7Q0000Pe600E2bb619T4v1uf0f0q011z6666ji8k8k3jSBKSJJAArriiiiii07JCABrzrrrrrrr00YrkqHrsrrrrjr005zrAqzrjzrrqr1jRjrqGGrrzsrsA099ijrABJJJIAzrrtirqrqjqixzsrAjrqjiqaqqysttAJqjikikrizrHtBJJAzArzrIsRCITKSS099ijrAJS____Qg99habbCAYrDzh00E0b4h8Qd3gQ5ho0000000000000hklDq00000000000000i4Ql5pSyHGg00000000004h8Qd000000000000000014j8y8y8x4h00000000000p2brFE_lldcNN3jjckgkQP13QQXRZddZZddZZdcMN1bh1Bl55555543hhhhhhhAl5555555l5555556x3QQP13kQP53QQOIfjjLnQQP13P13P13P144Mh1ji0mqfCOqf3wasKYGKD4NYGKDbLaHFUaKNiODbLaHFPRIKK-CGDcPha-HeJMzEOPbcsQu2JFvoJF7mJZqix7jhZydCzNkV56HWXbY3jbEi6vy5MJ0J6jlkR_nkicRf2rjkZuQkmq_zi_8SYx4zWAkkQug92KBCzMZ8Sq_gQzP4qKHLo8WyfmHG8ZSCKnHEY2ubQQvx6Ph-PmzkAhYNpmhIR_ZTihVdePjCLMs8U7RuocQa1FOXOGWsi_aHFOXOGWu3bIkIFOXOGWsZrbHLFGFPcQiLGPHs8WcIOP7d7wHqnSbqhRHvmAEhZIhWaD8ERvnpvwqpt2gPYgK5E5EOqGCLWWyhCFUjqqDHSyyPnYqnV6TA8AvkyU9sb8i0Ixqb8i0J50J58JltBJRWLOYLnx83DG1qa1qa1qa1qa1q90mywmywmCwmxjh-fzuqE5EE5EE5EE5EE5EF5EE5EE5FE5EO5rCRYz6RsyBpylSEQSLWGAXaq_lgszj0kRlr5dtk98idv14x9RwmqcUcCzfPGH2MfaKPcPcOCrbHOCqsITaVtcD0q8WFOsGyeGsDaEzGD9OG8ZlIHcHbFFO98Wp8Wp8TcFAVJFB8sQptWCkGg00))

All other art and assets were created by me