---
home: true
actionText: 开始阅读
actionLink: /trivial/caesar
footer: CC0 Licensed | Copyright © 2012-2019 Aster™
---


```bash
[lib]
crate-type = ["cdylib", "rlib"]



wasm-bindgen -V
cargo install wasm-bindgen-cli --version 0.2.68 --force
//wasm-pack build --target web && wasm-pack publish
wasm-pack build --target no-modules && wasm-pack publish
```


[![Netlify Status](https://api.netlify.com/api/v1/badges/2a90a2d1-6fe1-4298-8248-2f4f22f23837/deploy-status)](https://app.netlify.com/sites/crypto-moe/deploys)
