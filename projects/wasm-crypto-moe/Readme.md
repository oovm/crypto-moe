Marysue™ Encoding
=================

[lib]
crate-type = ["cdylib", "rlib"]


```bash
wasm-bindgen -V
cargo install wasm-bindgen-cli --version 0.2.68 --force
wasm-pack build --target web && wasm-pack publish
//wasm-pack build --target no-modules && wasm-pack publish
```
