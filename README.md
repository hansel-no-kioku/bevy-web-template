# Bevy web template

Template for a game application using [Bavy engine](https://bevyengine.org/) and [bevy_webgl2](https://github.com/mrk-its/bevy_webgl2)

## Preparation

### Install Rust and add target

Follow [Install Rust](https://www.rust-lang.org/tools/install) and

```shell
rustup target add wasm32-unknown-unknown
```

or

```shell
pkg install rust rust-std-wasm32
```

on Termux

### Crates

```shell
cargo install wasm-bindgen-cli
cargo install cargo-make
cargo install basic-http-server
```

## Run

```shell
cargo make run
```

and open http://127.0.0.1:4000 in browser
