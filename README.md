# rs-sc-dns
Numbat DNS smart contract, written in Rust



# Build

Temporarily until drtpy is updated, use the following to build:
```
./build-wasm.sh 
```
It will produce dns.wasm and user-mock.wasm (a dummy used in tests instead of the user API).

# Rust configuration

```
rustup toolchain install nightly
rustup target add wasm32-unknown-unknown
rustc --version
```

# Advanced

Wasm is built with this line:
```
cargo build --bin wasm --target=wasm32-unknown-unknown --release
```

To debug macros:
- wasm mode:
```
cargo +nightly rustc --bin wasm -- -Z unstable-options --pretty=expanded > demacroed.rs
```

- debug mode:
```
cargo +nightly rustc --lib -- -Z unstable-options --pretty=expanded > demacroed.rs
```

To check wasm size:
```
twiggy top -n 20 target/wasm32-unknown-unknown/release/wasm.wasm
```

To work with unpublished numbat-wasm crates, clone https://github.com/TerraDharitri/drt-rs-sdk in the same parent directory and replace dependencies in Cargo.toml with:
```
numbat-wasm = { path = "../../numbat-rs-wasm/numbat-wasm" }
numbat-wasm-node = { path = "../../numbat-rs-wasm/numbat-wasm-node" }
numbat-wasm-derive = { path = "../../numbat-rs-wasm/numbat-wasm-derive" }
```

And the same for debug:
```
numbat-wasm = { path = "../../../numbat-rs-wasm/numbat-wasm" }
numbat-wasm-debug = { path = "../../../numbat-rs-wasm/numbat-wasm-debug" }
```
