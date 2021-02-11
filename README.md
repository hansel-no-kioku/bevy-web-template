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

### Install crates

```shell
cargo install wasm-bindgen-cli cargo-generate cargo-make basic-http-server
```

## Create a project

```shell
cargo generate --git https://github.com/hansel-no-kioku/bevy-web-template.git --name my-project
```

## Run

```shell
cd my-project
cargo make test
```

and open http://127.0.0.1:4000 in browser

Publish on port 4000 with `cargo make run` (Linux only)

