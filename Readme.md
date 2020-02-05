Marysue™ Encoding
=================

- Encoding: UTF8 -> GB18030 -> BitXor -> BitXor -> BaseMap

```rust
extern crate crypto_marysue;
use crypto_marysue::{decode, encode};

#[test]
fn test_encoding() {
    let input = "力微任重久神疲, 再竭衰庸定不支.";
    let r1 = encode(input);
    let r2 = encode(input);
    debug_assert_ne!(r1, r2)
}

